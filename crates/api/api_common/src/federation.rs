pub use studycycle_db_schema::{
  newtypes::ActivityId,
  source::{
    federation_allowlist::FederationAllowList,
    federation_blocklist::FederationBlockList,
    federation_queue_state::FederationQueueState,
    instance::{Instance, InstanceActions},
  },
};
pub use studycycle_db_schema_file::{InstanceId, enums::FederationMode};
pub use studycycle_db_views_site::api::{
  GetFederatedInstances,
  GetFederatedInstancesKind,
  ResolveObject,
  UserBlockInstanceCommunitiesParams,
  UserBlockInstancePersonsParams,
};

pub mod administration {
  pub use studycycle_db_views_site::api::{AdminAllowInstanceParams, AdminBlockInstanceParams};
}
