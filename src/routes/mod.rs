mod config;
mod find_jobs;
mod health;

use crate::services::config::ConfigService;
use crate::services::job_match::JobMatchService;
use std::sync::Arc;
use warp::Filter;

pub fn route<JMS, CS>(
  job_match_service: Arc<JMS>,
  config_service: Arc<CS>,
) -> warp::filters::BoxedFilter<(impl warp::Reply,)>
where
  JMS: JobMatchService + Send + Sync + 'static,
  CS: ConfigService + Send + Sync + 'static,
{
  warp::path!("api" / ..)
    .and(
      find_jobs::route(job_match_service.clone(), config_service.clone())
        .or(health::route(config_service))
        .or(config::route(job_match_service)),
    )
    .boxed()
}
