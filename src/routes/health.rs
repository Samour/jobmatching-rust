use crate::services::config::ConfigService;
use serde::Serialize;
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize)]
struct HealthResponse {
  #[serde(rename = "appName")]
  app_name: String,
}

pub fn route<T>(config_service: Arc<T>) -> warp::filters::BoxedFilter<(impl warp::Reply,)>
where
  T: ConfigService + Send + Sync + 'static,
{
  warp::path("health")
    .map(move || {
      warp::reply::json(&HealthResponse {
        app_name: config_service.get_config().app_name.clone(),
      })
    })
    .boxed()
}
