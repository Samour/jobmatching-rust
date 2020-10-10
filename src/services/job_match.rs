use crate::dto::{
  JobDto, JobScoreDto, RuleConfigDto, RuleResultDto, StackDiagnosisResponse, WorkerDto,
};
use crate::engine::match_rating::MatchRating;
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::Arc;
use std::time::Instant;
use warp::reject::Rejection;

#[derive(Clone)]
struct JobScore {
  job: JobDto,
  rating: f64,
  details: Vec<RuleResultDto>,
}

fn job_score_cmp(a: &JobScore, b: &JobScore) -> Ordering {
  if a.rating > b.rating {
    Ordering::Less
  } else if a.rating < b.rating {
    Ordering::Greater
  } else {
    Ordering::Equal
  }
}

#[async_trait]
pub trait JobMatchService {
  fn get_rule_configs(&self) -> Vec<RuleConfigDto>;
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
  match_ratings: Arc<Vec<Box<dyn MatchRating + Send + Sync>>>,
  rest_repository: Arc<dyn RestRepository + Send + Sync>,
}

impl JobMatchServiceImpl {
  pub fn new(
    match_ratings: Arc<Vec<Box<dyn MatchRating + Send + Sync>>>,
    rest_repository: Arc<dyn RestRepository + Send + Sync>,
  ) -> JobMatchServiceImpl {
    JobMatchServiceImpl {
      match_ratings,
      rest_repository,
    }
  }

  fn score_job_for_worker(&self, worker: &WorkerDto, job: JobDto) -> JobScore {
    log::debug!(
      "Calculating score for Worker {} and Job {}",
      worker.user_id,
      job.job_id
    );
    let mut score = JobScore {
      job,
      rating: 0.0,
      details: Vec::new(),
    };
    for match_rating in self.match_ratings.iter() {
      let result = match_rating.determine_rating(worker, &score.job);
      if result.rating < 0.0 || score.rating < 0.0 {
        score.rating = -1.0;
      } else {
        score.rating += result.rating;
      }
      score.details.push(RuleResultDto {
        rule_name: String::from(match_rating.get_name()),
        weight: match_rating.get_weight(),
        rating: result.rating,
        metrics: result.metrics,
      })
    }

    log::debug!("Score calculated: {}", score.rating);
    score
  }

  async fn get_and_score_jobs(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobScore>, Rejection> {
    log::debug!("Calculating jobs for Worker {}", worker_id);
    let worker = self.rest_repository.find_worker_by_id(worker_id).await?;
    if let None = worker {
      log::warn!("Could not find worker {}", worker_id);
      return Err(warp::reject::custom(BadRequestError::new()));
    }
    let worker = worker.unwrap();

    let mut jobs: Vec<JobScore> = self
      .rest_repository
      .find_all_jobs()
      .await?
      .iter()
      .map(|job| self.score_job_for_worker(&worker, job.clone()))
      .collect();
    jobs.sort_by(job_score_cmp);

    log::debug!("Job scoring complete");
    Ok(jobs[..(job_limit as usize)].to_vec())
  }
}

#[async_trait]
impl JobMatchService for JobMatchServiceImpl {
  fn get_rule_configs(&self) -> Vec<RuleConfigDto> {
    self
      .match_ratings
      .iter()
      .map(|r| RuleConfigDto {
        name: String::from(r.get_name()),
        weight: r.get_weight(),
      })
      .collect()
  }

  async fn rate_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<StackDiagnosisResponse, Rejection> {
    let start = Instant::now();
    let jobs = self
      .get_and_score_jobs(worker_id, job_limit)
      .await?
      .iter()
      .map(|j| JobScoreDto {
        job_id: j.job.job_id,
        rating: j.rating,
        rule_results: j.details.clone(),
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
    let jobs = self
      .get_and_score_jobs(worker_id, job_limit)
      .await?
      .iter()
      .map(|j| j.job.clone())
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Job stack calculated in {}ms", calculation_time_ms);

    Ok(jobs)
  }
}
