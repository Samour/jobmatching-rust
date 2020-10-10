use crate::dto::{JobDto, WorkerDto};

pub trait MatchRating {
  fn determine_rating(&self, worker: &WorkerDto, job: &JobDto) -> f64;
}
