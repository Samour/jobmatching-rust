use crate::dto::{JobDto, JobScoreDto, RuleConfigDto, RuleResultDto, StackDiagnosisResponse};
use crate::engine::config::{EvaluationConfig, EvaluationContext};
use crate::engine::match_rating::MatchRating;
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::Arc;
use std::time::Instant;
use warp::reject::Rejection;

#[derive(Clone)]
struct MatchScore {
  rating: f64,
  details: Vec<RuleResultDto>,
}

fn match_score_cmp<T>(a: &(T, MatchScore), b: &(T, MatchScore)) -> Ordering {
  if a.1.rating > b.1.rating {
    Ordering::Less
  } else if a.1.rating < b.1.rating {
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

  fn score_job_for_worker(&self, ctx: &EvaluationContext) -> MatchScore {
    log::debug!(
      "Calculating score for Worker {} and Job {}",
      ctx.worker.user_id,
      ctx.job.job_id
    );
    let mut score = MatchScore {
      rating: 0.0,
      details: Vec::new(),
    };
    for match_rating in self.match_ratings.iter() {
      let result = match_rating.determine_rating(&ctx);
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

  // TODO if we split this into two methods (async get) and (sync score), we can cut out 1 of the clones in the
  // trait impl fns
  async fn get_and_score_jobs(
    &self,
    worker_id: u32,
    job_limit: u32,
    config: &EvaluationConfig,
  ) -> Result<Vec<(JobDto, MatchScore)>, Rejection> {
    log::debug!("Calculating jobs for Worker {}", worker_id);
    let worker = self.rest_repository.find_worker_by_id(worker_id).await?;
    if let None = worker {
      log::warn!("Could not find worker {}", worker_id);
      return Err(warp::reject::custom(BadRequestError::new()));
    }
    let worker = worker.unwrap();

    let jobs = self.rest_repository.find_all_jobs().await?;
    let mut jobs: Vec<(&JobDto, MatchScore)> = jobs
      .iter()
      .map(|job| {
        (
          job,
          self.score_job_for_worker(&EvaluationContext::new(&worker, job, config)),
        )
      })
      .collect();
    // TODO instead of scoring & collecting every job before sorting, instead push each element onto a max heap
    // as they are scored. Set the max heap size = job_limit; after each push, if the heap exceeds that size, drop
    // the smallest value. We can also store the current lowest score in the heap to short-circuit pushing then dropping
    // too-small values.
    // This will drastically cut down on the number of comparisons that need to be made compared to sorting the full
    // list. Additionally, we can reduce the memory footprint somewhat (althoug this is less relevent as find_all_jobs
    // already loads all data into memory anyway)
    jobs.sort_by(match_score_cmp);

    log::debug!("Job scoring complete");
    Ok(
      jobs[..(job_limit as usize)]
        .iter()
        .map(|j| (j.0.clone(), j.1.clone()))
        .collect(),
    )
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
    let config = EvaluationConfig {
      with_diagnosis: true,
      short_circuit_failures: false,
    };
    let jobs = self
      .get_and_score_jobs(worker_id, job_limit, &config)
      .await?
      .iter()
      .map(|j| JobScoreDto {
        job_id: j.0.job_id,
        rating: j.1.rating,
        rule_results: j.1.details.clone(),
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
    let config = EvaluationConfig {
      with_diagnosis: false,
      short_circuit_failures: true,
    };
    let jobs = self
      .get_and_score_jobs(worker_id, job_limit, &config)
      .await?
      .iter()
      .map(|j| j.0.clone())
      .collect();
    let calculation_time_ms = start.elapsed().as_millis();
    log::debug!("Job stack calculated in {}ms", calculation_time_ms);

    Ok(jobs)
  }
}
