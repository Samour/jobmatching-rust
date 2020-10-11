use super::config::EvaluationContext;
use super::match_rating::{MatchRating, RatingResult};
use std::collections::HashMap;

pub struct JobPositions {
  weight_value: f64,
}

impl JobPositions {
  pub fn new(weight: f64) -> JobPositions {
    JobPositions {
      weight_value: weight,
    }
  }
}

impl MatchRating for JobPositions {
  fn get_name(&self) -> &str {
    "JobPositions"
  }

  fn get_weight(&self) -> f64 {
    self.weight_value
  }

  fn determine_rating(&self, ctx: &EvaluationContext) -> RatingResult {
    log::debug!(
      "Running rule {} for Worker {} and Job {}",
      self.get_name(),
      ctx.worker.user_id,
      ctx.job.job_id
    );
    let mut metrics: HashMap<String, f64> = HashMap::new();
    if ctx.config.with_diagnosis {
      metrics.insert(
        String::from("workersRequired"),
        ctx.job.workers_required as f64,
      );
    }

    let rating = self.weight_value * ctx.job.workers_required as f64;
    log::debug!("Rule {} completed with rating {}", self.get_name(), rating);
    RatingResult { rating, metrics }
  }
}
