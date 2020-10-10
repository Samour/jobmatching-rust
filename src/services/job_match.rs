use crate::dto::JobDto;
use crate::engine::match_rating::MatchRating;
use crate::errors::bad_request::BadRequestError;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::cmp::Ordering;
use std::sync::Arc;
use warp::reject::Rejection;

struct JobScore<'a> {
  job: &'a JobDto,
  rating: f64,
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
}

#[async_trait]
impl JobMatchService for JobMatchServiceImpl {
  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Rejection> {
    let worker_opt = self.rest_repository.find_worker_by_id(worker_id).await?;
    if let None = worker_opt {
      return Err(warp::reject::custom(BadRequestError::new()));
    }
    let worker = worker_opt.unwrap();

    let all_jobs = self.rest_repository.find_all_jobs().await?;
    let mut matched_jobs: Vec<JobScore> = all_jobs
      .iter()
      .map(|job| {
        let mut score = JobScore { job, rating: 0.0 };
        for match_rating in self.match_ratings.iter() {
          let rating = match_rating.determine_rating(&worker, job);
          if rating < 0.0 {
            return None;
          } else {
            score.rating += rating;
          }
        }
        Some(score)
      })
      .filter(|o| o.is_some())
      .map(|o| o.unwrap())
      .collect();
    matched_jobs.sort_by(job_score_cmp);

    Ok(
      matched_jobs[..(job_limit as usize)]
        .iter()
        .map(|j| j.job)
        .cloned()
        .collect(),
    )
  }
}
