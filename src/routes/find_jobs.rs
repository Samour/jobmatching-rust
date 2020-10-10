use crate::services::config::ConfigService;
use crate::services::job_match::JobMatchService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct FindJobsQuery {
  #[serde(rename = "workerId")]
  worker_id: u32,
}

pub fn route<T, C>(
  job_match_service: Arc<T>,
  config_service: Arc<C>,
) -> warp::filters::BoxedFilter<(impl warp::Reply,)>
where
  T: JobMatchService + Send + Sync + 'static,
  C: ConfigService + Send + Sync + 'static,
{
  warp::path!("findJobsForWorker")
    .and(warp::query().map(|q: FindJobsQuery| q.worker_id))
    .and_then(move |worker_id| {
      let jms_local = job_match_service.clone();
      let cs_local = config_service.clone();
      async move {
        let r_jobs = jms_local
          .find_best_jobs_for_worker(worker_id, cs_local.get_config().jobs_to_return)
          .await;
        match r_jobs {
          Ok(jobs) => Ok(warp::reply::json(&jobs)),
          Err(_) => Err(warp::reject()),
        }
      }
    })
    .boxed()
}
