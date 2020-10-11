use crate::services::config::ConfigService;
use crate::services::worker_match::WorkerMatchService;
use serde::Deserialize;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::Filter;

#[derive(Deserialize)]
struct FindWorkersQuery {
  #[serde(rename = "jobId")]
  job_id: u32,
}

pub fn route<WMS, CS>(
  worker_match_service: Arc<WMS>,
  config_service: Arc<CS>,
) -> BoxedFilter<(impl warp::Reply,)>
where
  WMS: WorkerMatchService + Send + Sync + 'static,
  CS: ConfigService + Send + Sync + 'static,
{
  let wms1 = worker_match_service.clone();
  let cs1 = config_service.clone();
  let find_workers = warp::path!("findWorkersForJob")
    .and(warp::get())
    .and(warp::query().map(|q: FindWorkersQuery| q.job_id))
    .and_then(move |job_id| {
      let wms_local = wms1.clone();
      let cs_local = cs1.clone();
      async move {
        wms_local
          .find_best_workers_for_job(job_id, cs_local.get_config().workers_to_return)
          .await
          .map(|w| warp::reply::json(&w))
      }
    });

  let diagnose_workers = warp::path!("diagnoseWorkers")
    .and(warp::get())
    .and(warp::query().map(|q: FindWorkersQuery| q.job_id))
    .and_then(move |job_id| {
      let wms_local = worker_match_service.clone();
      let cs_local = config_service.clone();
      async move {
        wms_local
          .rate_workers_for_job(job_id, cs_local.get_config().workers_to_return)
          .await
          .map(|w| warp::reply::json(&w))
      }
    });

  find_workers.or(diagnose_workers).boxed()
}
