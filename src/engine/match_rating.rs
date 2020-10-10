use crate::dto::{JobDto, WorkerDto};
use std::collections::HashMap;

pub struct RatingResult {
  pub rating: f64,
  pub metrics: HashMap<String, f64>,
}

pub trait MatchRating {
  fn get_name(&self) -> &str;
  fn get_weight(&self) -> f64;
  fn determine_rating(&self, worker: &WorkerDto, job: &JobDto) -> RatingResult;
}
