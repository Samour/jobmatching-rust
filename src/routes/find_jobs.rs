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
  let jms1 = job_match_service.clone();
  let cs1 = config_service.clone();
  let find_jobs = warp::path!("findJobsForWorker")
    .and(warp::get())
    .and(warp::query().map(|q: FindJobsQuery| q.worker_id))
    .and_then(move |worker_id| {
      let jms_local = jms1.clone();
      let cs_local = cs1.clone();
      async move {
        jms_local
          .find_best_jobs_for_worker(worker_id, cs_local.get_config().jobs_to_return)
          .await
          .map(|j| warp::reply::json(&j))
      }
    });

  let diagnose_stack = warp::path!("diagnoseStack")
    .and(warp::get())
    .and(warp::query().map(|q: FindJobsQuery| q.worker_id))
    .and_then(move |worker_id| {
      let jms_local = job_match_service.clone();
      let cs_local = config_service.clone();
      async move {
        jms_local
          .rate_jobs_for_worker(worker_id, cs_local.get_config().jobs_to_return)
          .await
          .map(|j| warp::reply::json(&j))
      }
    });

  find_jobs.or(diagnose_stack).boxed()
}
