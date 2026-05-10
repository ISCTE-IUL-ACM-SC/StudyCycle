use crate::context::StudyCycleContext;
use anyhow::anyhow;
use extism::{
  FromBytesOwned,
  Manifest,
  PluginBuilder,
  Pool,
  PoolPlugin,
  ToBytes,
  Wasm,
  WasmMetadata,
};
use extism_convert::Json;
use extism_manifest::HttpRequest;
use studycycle_db_schema::source::{notification::Notification, person::Person};
use studycycle_db_views_notification::NotificationView;
use studycycle_db_views_registration_applications::api::CaptchaAnswer;
use studycycle_db_views_site::api::{CaptchaResponse, PluginMetadata};
use studycycle_diesel_utils::traits::Crud;
use studycycle_utils::{
  VERSION,
  error::{StudyCycleError, StudyCycleErrorType, StudyCycleResult},
  settings::{SETTINGS, structs::PluginSettings},
};
use serde::{Deserialize, Serialize};
use std::{
  env::var,
  ops::Deref,
  path::PathBuf,
  sync::{LazyLock, OnceLock},
  time::Duration,
};
use tokio::task::spawn_blocking;
use tracing::{error, warn};
use url::Url;

const GET_PLUGIN_TIMEOUT: Duration = Duration::from_secs(1);

/// Call a plugin hook without rewriting data
pub fn plugin_hook_after<T>(name: &'static str, data: &T)
where
  T: Clone + Serialize + for<'b> Deserialize<'b> + Sync + Send + 'static,
{
  let plugins = StudyCyclePlugins::get_or_init();
  if !plugins.function_exists(name) {
    return;
  }

  let data = data.clone();
  spawn_blocking(move || run_plugin_hook_after(name, data));
}

/// Calls plugin hook for the given notifications Loads additional data via
/// NotificationView, but only if a plugin is active.
pub async fn plugin_hook_notification(
  notifications: Vec<Notification>,
  context: &StudyCycleContext,
) -> StudyCycleResult<()> {
  let name = "notification_after_create";
  let plugins = StudyCyclePlugins::get_or_init();
  if !plugins.function_exists(name) {
    return Ok(());
  }

  for n in notifications {
    let person = Person::read(&mut context.pool(), n.recipient_id).await?;
    let view = NotificationView::read(&mut context.pool(), n.id, &person).await?;
    spawn_blocking(move || run_plugin_hook_after(name, view));
  }
  Ok(())
}

pub async fn plugin_get_captcha() -> StudyCycleResult<CaptchaResponse> {
  call_captcha_plugin("get_captcha", ()).await
}

pub async fn plugin_validate_captcha(answer: String, uuid: String) -> StudyCycleResult<()> {
  call_captcha_plugin("validate_captcha", CaptchaAnswer { answer, uuid }).await
}

async fn call_captcha_plugin<
  'a,
  T: ToBytes<'a> + Send + 'static,
  R: FromBytesOwned + Send + 'static,
>(
  name: &'static str,
  params: T,
) -> StudyCycleResult<R> {
  let plugins = StudyCyclePlugins::get_or_init();
  let Some(captcha_plugin) = plugins.captcha_plugin else {
    return Err(StudyCycleErrorType::PluginError("plugin not loaded".to_string()).into());
  };

  spawn_blocking(move || {
    if let Some(mut p) = captcha_plugin.pool.get(GET_PLUGIN_TIMEOUT)? {
      let res = p
        .call(name, params)
        .map_err(|e| StudyCycleErrorType::PluginError(e.to_string()))?;
      return Ok(res);
    }
    Err(StudyCycleErrorType::PluginError("plugin not loaded".to_string()).into())
  })
  .await?
}

pub fn is_captcha_plugin_loaded() -> bool {
  StudyCyclePlugins::get_or_init().captcha_plugin.is_some()
}

fn run_plugin_hook_after<T>(name: &'static str, data: T) -> StudyCycleResult<()>
where
  T: Clone + Serialize + for<'b> Deserialize<'b>,
{
  let plugins = StudyCyclePlugins::get_or_init();
  for p in plugins.plugins {
    if let Some(mut plugin) = p.get(name)? {
      let params: Json<T> = data.clone().into();
      plugin
        .call::<Json<T>, ()>(name, params)
        .map_err(|e| StudyCycleErrorType::PluginError(e.to_string()))?;
    }
  }
  Ok(())
}

/// Call a plugin hook which can rewrite data
pub async fn plugin_hook_before<T>(name: &'static str, data: T) -> StudyCycleResult<T>
where
  T: Clone + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static,
{
  let plugins = StudyCyclePlugins::get_or_init();
  if !plugins.function_exists(name) {
    return Ok(data);
  }

  spawn_blocking(move || {
    let mut res: Json<T> = data.into();
    for p in plugins.plugins {
      if let Some(mut plugin) = p.get(name)? {
        let r = plugin
          .call(name, res)
          .map_err(|e| StudyCycleErrorType::PluginError(e.to_string()))?;
        res = r;
      }
    }
    Ok::<_, StudyCycleError>(res.0)
  })
  .await?
}

pub fn plugin_metadata() -> Vec<PluginMetadata> {
  static METADATA: OnceLock<Vec<PluginMetadata>> = OnceLock::new();
  if let Some(m) = METADATA.get() {
    m.clone()
  } else {
    // Loading metadata can take multiple seconds. Do this in background task to avoid blocking
    // /api/v4/site endpoint.
    std::thread::spawn(|| {
      METADATA.get_or_init(|| {
        let mut metadata = vec![];
        for plugin in StudyCyclePlugins::get_or_init().plugins {
          let run = match plugin.pool.get(GET_PLUGIN_TIMEOUT) {
            Ok(p) => p,
            Err(e) => {
              error!("Failed to load plugin {}: {e}", plugin.filename);
              continue;
            }
          };
          let m = run.and_then(|mut run| run.call("metadata", 0).ok());
          if let Some(m) = m {
            metadata.push(m);
          } else {
            // Failed to load plugin metadata, use placeholder
            metadata.push(PluginMetadata {
              name: plugin.filename,
              url: None,
              description: None,
            });
          }
        }
        metadata
      });
    });
    // Return empty metadata until loading is finished
    vec![]
  }
}

#[derive(Clone)]
struct StudyCyclePlugins {
  plugins: Vec<StudyCyclePlugin>,
  captcha_plugin: Option<StudyCyclePlugin>,
}

#[derive(Clone)]
struct StudyCyclePlugin {
  pool: Pool,
  filename: String,
}

impl StudyCyclePlugin {
  fn init(settings: PluginSettings) -> StudyCycleResult<Self> {
    let hash = if cfg!(debug_assertions) || var("DANGER_PLUGIN_SKIP_HASH_CHECK").is_ok() {
      None
    } else {
      // if no hash was provided in config, set a dummy value here to enforce hash check
      Some(settings.hash.unwrap_or_else(|| "dummy".to_string()))
    };
    let meta = WasmMetadata { hash, name: None };
    let (wasm, filename) = if settings.file.starts_with("http") {
      let name: Option<String> = Url::parse(&settings.file)?
        .path_segments()
        .and_then(|mut p| p.next_back())
        .map(std::string::ToString::to_string);
      let req = HttpRequest {
        url: settings.file.clone(),
        headers: Default::default(),
        method: None,
      };
      (Wasm::Url { req, meta }, name)
    } else {
      let path = PathBuf::from(settings.file.clone());
      let name: Option<String> = path.file_name().map(|n| n.to_string_lossy().to_string());
      (Wasm::File { path, meta }, name)
    };
    let mut manifest = Manifest {
      wasm: vec![wasm],
      config: settings.config,
      allowed_hosts: settings.allowed_hosts,
      memory: Default::default(),
      allowed_paths: None,
      timeout_ms: None,
    };
    manifest.config.insert(
      "studycycle_url".to_string(),
      format!("http://{}:{}/", SETTINGS.bind, SETTINGS.port),
    );
    manifest
      .config
      .insert("studycycle_version".to_string(), VERSION.to_string());
    let builder = move || PluginBuilder::new(manifest.clone()).with_wasi(true).build();
    let pool = Pool::new(builder);
    Ok(StudyCyclePlugin {
      pool,
      filename: filename.unwrap_or(settings.file),
    })
  }

  #[expect(clippy::if_then_some_else_none)]
  fn get(&self, name: &'static str) -> StudyCycleResult<Option<PoolPlugin>> {
    let p = self
      .pool
      .get(GET_PLUGIN_TIMEOUT)?
      .ok_or(anyhow!("plugin timeout"))?;

    Ok(if p.function_exists(name) {
      Some(p)
    } else {
      None
    })
  }
}

impl StudyCyclePlugins {
  /// Load and initialize all plugins
  fn get_or_init() -> Self {
    static PLUGINS: LazyLock<StudyCyclePlugins> = LazyLock::new(|| {
      let mut plugins: Vec<_> = SETTINGS
        .plugins
        .iter()
        .flat_map(|p| {
          StudyCyclePlugin::init(p.clone())
            .inspect_err(|e| warn!("Failed to load plugin {}: {e}", p.file))
            .ok()
        })
        .collect();

      let mut captcha_plugin = None;
      for (i, p) in plugins.iter().enumerate() {
        let is_captcha = p
          .pool
          .function_exists("validate_captcha", GET_PLUGIN_TIMEOUT)
          .unwrap_or_default()
          && p
            .pool
            .function_exists("validate_captcha", GET_PLUGIN_TIMEOUT)
            .unwrap_or_default();
        if is_captcha {
          captcha_plugin = Some(plugins.remove(i));
          break;
        }
      }

      // Need to put captcha plugin back in so it can be shown in the active plugins list.
      if let Some(captcha_plugin) = &captcha_plugin {
        plugins.push(captcha_plugin.clone());
      }
      StudyCyclePlugins {
        plugins,
        captcha_plugin,
      }
    });
    PLUGINS.deref().clone()
  }

  /// Return early if no plugin is loaded for the given hook name
  fn function_exists(&self, name: &'static str) -> bool {
    self.plugins.iter().any(|p| {
      p.pool
        .function_exists(name, GET_PLUGIN_TIMEOUT)
        .unwrap_or(false)
    })
  }
}
