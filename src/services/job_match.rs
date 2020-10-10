use super::rules::{MatchScore, RulesService};
use crate::dto::{JobDto, JobScoreDto, StackDiagnosisResponse, WorkerDto};
use crate::engine::config::{EvaluationConfig, EvaluationContext};
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use warp::reject::Rejection;

#[async_trait]
pub trait JobMatchService {
  async fn rate_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<StackDiagnosisResponse, Rejection>;
  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Rejection>;
}

pub struct JobMatchServiceImpl {
  rules_service: Arc<dyn RulesService + Send + Sync>,
  rest_repository: Arc<dyn RestRepository + Send + Sync>,
}

impl JobMatchServiceImpl {
  pub fn new(
    rules_service: Arc<dyn RulesService + Send + Sync>,
    rest_repository: Arc<dyn RestRepository + Send + Sync>,
  ) -> JobMatchServiceImpl {
    JobMatchServiceImpl {
      rules_service,
      rest_repository,
    }
  }

  async fn load_data(&self, worker_id: u32) -> Result<(WorkerDto, Vec<JobDto>), Rejection> {
    let (worker, jobs) = tokio::join!(
      self.rest_repository.find_worker_by_id(worker_id),
      self.rest_repository.find_all_jobs()
    );
    match worker? {
      Some(w) => Ok((w, jobs?)),
      None => {
        log::warn!("Could not find worker {}", worker_id);
        Err(warp::reject::custom(BadRequestError::new()))
      }
    }
  }

  fn score_jobs<'a>(
    &self,
    worker: &WorkerDto,
    jobs: &'a Vec<JobDto>,
    job_limit: u32,
    config: &EvaluationConfig,
  ) -> Vec<(&'a JobDto, MatchScore)> {
    log::debug!("Calculating jobs for Worker {}", worker.user_id);
    self
      .rules_service
      .score_entries(
        &jobs
          .iter()
          .map(|j| EvaluationContext::new(worker, j, config))
          .collect(),
        job_limit,
      )
      .iter()
      .map(|r| (r.0.job, r.1.clone()))
      .collect()
  }
}

#[async_trait]
impl JobMatchService for JobMatchServiceImpl {
  async fn rate_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<StackDiagnosisResponse, Rejection> {
    let start = Instant::now();
    let (worker, jobs) = self.load_data(worker_id).await?;

    let config = EvaluationConfig {
      with_diagnosis: true,
      short_circuit_failures: false,
    };
    let jobs = self
      .score_jobs(&worker, &jobs, job_limit, &config)
      .into_iter()
      .map(|j| JobScoreDto {
        job_id: j.0.job_id,
        rating: j.1.rating,
        rule_results: j.1.details,
      })
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Diagnosis calculated in {}ms", calculation_time_ms);

    Ok(StackDiagnosisResponse {
      jobs,
      calculation_time_ms,
    })
  }

  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Rejection> {
    let start = Instant::now();
    let (worker, jobs) = self.load_data(worker_id).await?;

    let config = EvaluationConfig {
      with_diagnosis: false,
      short_circuit_failures: true,
    };
    let jobs = self
      .score_jobs(&worker, &jobs, job_limit, &config)
      .iter()
      .map(|j| j.0.clone())
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Job stack calculated in {}ms", calculation_time_ms);

    Ok(jobs)
  }
}
