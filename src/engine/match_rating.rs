use std::collections::HashMap;
use super::config::EvaluationContext;

pub struct RatingResult {
  pub rating: f64,
  pub metrics: HashMap<String, f64>,
}

pub trait MatchRating {
  fn get_name(&self) -> &str;
  fn get_weight(&self) -> f64;
  fn determine_rating(&self, ctx: &EvaluationContext) -> RatingResult;
}
