use crate::services::config::ConfigService;
use crate::services::job_match::JobMatchService;
use serde::Deserialize;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::Filter;

#[derive(Deserialize)]
struct FindJobsQuery {
  #[serde(rename = "workerId")]
  worker_id: u32,
}

#[derive(Deserialize)]
struct DiagnoseJobQuery {
  #[serde(rename = "workerId")]
  worker_id: u32,
  #[serde(rename = "jobId")]
  job_id: u32,
}

pub fn route<JMS, CS>(
  job_match_service: Arc<JMS>,
  config_service: Arc<CS>,
) -> BoxedFilter<(impl warp::Reply,)>
where
  JMS: JobMatchService + Send + Sync + 'static,
  CS: ConfigService + Send + Sync + 'static,
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

  let jms2 = job_match_service.clone();
  let diagnose_stack = warp::path!("diagnoseStack")
    .and(warp::get())
    .and(warp::query().map(|q: FindJobsQuery| q.worker_id))
    .and_then(move |worker_id| {
      let jms_local = jms2.clone();
      let cs_local = config_service.clone();
      async move {
        jms_local
          .rate_jobs_for_worker(worker_id, cs_local.get_config().jobs_to_return)
          .await
          .map(|j| warp::reply::json(&j))
      }
    });

  let diagnose_job = warp::path!("diagnoseJob")
    .and(warp::get())
    .and(warp::query().map(|q: DiagnoseJobQuery| q.worker_id))
    .and(warp::query().map(|q: DiagnoseJobQuery| q.job_id))
    .and_then(move |worker_id, job_id| {
      let jms_local = job_match_service.clone();
      async move {
        jms_local
          .rate_job_for_worker(worker_id, job_id)
          .await
          .map(|j| warp::reply::json(&j))
      }
    });

  find_jobs.or(diagnose_stack).or(diagnose_job).boxed()
}
