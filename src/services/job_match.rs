use crate::dto::{JobDto, JobScoreDto, RuleConfigDto, RuleResultDto, WorkerDto};
use crate::engine::match_rating::MatchRating;
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::Arc;
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
  ) -> Result<Vec<JobScoreDto>, Rejection>;
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

    score
  }

  async fn get_and_score_jobs(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobScore>, Rejection> {
    let worker = self.rest_repository.find_worker_by_id(worker_id).await?;
    if let None = worker {
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
  ) -> Result<Vec<JobScoreDto>, Rejection> {
    Ok(
      self
        .get_and_score_jobs(worker_id, job_limit)
        .await?
        .iter()
        .map(|j| JobScoreDto {
          job_id: j.job.job_id,
          rating: j.rating,
          rule_results: j.details.clone(),
        })
        .collect(),
    )
  }

  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Rejection> {
    Ok(
      self
        .get_and_score_jobs(worker_id, job_limit)
        .await?
        .iter()
        .map(|j| j.job.clone())
        .collect(),
    )
  }
}
