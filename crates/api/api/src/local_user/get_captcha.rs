use actix_web::{
  HttpResponse,
  HttpResponseBuilder,
  http::{
    StatusCode,
    header::{CacheControl, CacheDirective},
  },
  web::Json,
};
use studycycle_api_utils::plugins::{is_captcha_plugin_loaded, plugin_get_captcha};
use studycycle_db_views_site::api::GetCaptchaResponse;
use studycycle_utils::error::StudyCycleResult;

pub async fn get_captcha() -> StudyCycleResult<HttpResponse> {
  let mut res = HttpResponseBuilder::new(StatusCode::OK);
  res.insert_header(CacheControl(vec![CacheDirective::NoStore]));

  if !is_captcha_plugin_loaded() {
    return Ok(res.json(Json(GetCaptchaResponse { ok: None })));
  }

  let captcha = GetCaptchaResponse {
    ok: Some(plugin_get_captcha().await?),
  };
  Ok(res.json(Json(captcha)))
}
