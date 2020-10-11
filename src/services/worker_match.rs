use super::rules::{MatchScore, RulesService};
use crate::dto::{JobDto, MatchScoreDto, WorkerDto, WorkersDiagnosisResponse};
use crate::engine::config::{EvaluationConfig, EvaluationContext};
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use warp::Rejection;

#[async_trait]
pub trait WorkerMatchService {
  async fn rate_workers_for_job(
    &self,
    job_id: u32,
    worker_limit: u32,
  ) -> Result<WorkersDiagnosisResponse, Rejection>;
  async fn find_best_workers_for_job(
    &self,
    job_id: u32,
    worker_limit: u32,
  ) -> Result<Vec<WorkerDto>, Rejection>;
  async fn count_matching_workers(&self, job_id: u32) -> Result<usize, Rejection>;
}

pub struct WorkerMatchServiceImpl {
  rules_service: Arc<dyn RulesService + Send + Sync>,
  rest_repository: Arc<dyn RestRepository + Send + Sync>,
}

impl WorkerMatchServiceImpl {
  pub fn new(
    rules_service: Arc<dyn RulesService + Send + Sync>,
    rest_repository: Arc<dyn RestRepository + Send + Sync>,
  ) -> WorkerMatchServiceImpl {
    WorkerMatchServiceImpl {
      rules_service,
      rest_repository,
    }
  }

  async fn load_data(&self, job_id: u32) -> Result<(JobDto, Vec<WorkerDto>), Rejection> {
    let (job, workers) = tokio::join!(
      self.rest_repository.find_job_by_id(job_id),
      self.rest_repository.find_all_workers(),
    );
    match job? {
      Some(j) => Ok((j, workers?)),
      None => {
        log::warn!("Could not find job {}", job_id);
        Err(warp::reject::custom(BadRequestError::new()))
      }
    }
  }

  fn score_workers<'a>(
    &self,
    job: &JobDto,
    workers: &'a Vec<WorkerDto>,
    worker_limit: u32,
    config: &EvaluationConfig,
  ) -> Vec<(&'a WorkerDto, MatchScore)> {
    log::debug!("Calculating workers for Job {}", job.job_id);
    self
      .rules_service
      .score_entries(
        &workers
          .iter()
          .map(|w| EvaluationContext::new(w, job, config))
          .collect(),
        worker_limit,
      )
      .into_iter()
      .map(|r| (r.0.worker, r.1))
      .collect()
  }
}

#[async_trait]
impl WorkerMatchService for WorkerMatchServiceImpl {
  async fn rate_workers_for_job(
    &self,
    job_id: u32,
    worker_limit: u32,
  ) -> Result<WorkersDiagnosisResponse, Rejection> {
    let start = Instant::now();
    let (job, workers) = self.load_data(job_id).await?;

    let config = EvaluationConfig {
      with_diagnosis: true,
      short_circuit_failures: false,
    };
    let workers = self
      .score_workers(&job, &workers, worker_limit, &config)
      .into_iter()
      .map(|w| MatchScoreDto {
        worker_id: w.0.user_id,
        job_id,
        rating: w.1.rating,
        rule_results: w.1.details,
      })
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Worker diagnosis calculated in {}ms", calculation_time_ms);

    Ok(WorkersDiagnosisResponse {
      workers,
      calculation_time_ms,
    })
  }

  async fn find_best_workers_for_job(
    &self,
    job_id: u32,
    worker_limit: u32,
  ) -> Result<Vec<WorkerDto>, Rejection> {
    let start = Instant::now();
    let (job, workers) = self.load_data(job_id).await?;
    let config = EvaluationConfig {
      with_diagnosis: false,
      short_circuit_failures: true,
    };
    let workers = self
      .score_workers(&job, &workers, worker_limit, &config)
      .iter()
      .map(|w| w.0.clone())
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Matching workers calculated in {}ms", calculation_time_ms);

    Ok(workers)
  }

  async fn count_matching_workers(&self, job_id: u32) -> Result<usize, Rejection> {
    let start = Instant::now();
    let (job, workers) = self.load_data(job_id).await?;
    let config = EvaluationConfig {
      with_diagnosis: false,
      short_circuit_failures: true,
    };
    let count = self.rules_service.count_satisfied(
      &workers
        .iter()
        .map(|w| EvaluationContext::new(w, &job, &config))
        .collect(),
    );
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Matching workers counted in {}ms", calculation_time_ms);

    Ok(count)
  }
}
