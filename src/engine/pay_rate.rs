use super::config::EvaluationContext;
use super::match_rating::{MatchRating, RatingResult};
use std::collections::HashMap;

pub struct PayRate {
  weight_value: f64,
}

impl PayRate {
  pub fn new(weight: f64) -> PayRate {
    PayRate {
      weight_value: weight,
    }
  }
}

impl MatchRating for PayRate {
  fn get_name(&self) -> &str {
    "PayRate"
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
    let rate = ctx.job.bill_rate.replace("$", "").parse::<f64>();
    if let Err(e) = rate {
      log::error!("Error parsing job.bill_rate {:?}", e);
      return RatingResult {
        rating: 0.0,
        metrics: HashMap::new(),
      };
    }
    let rate = rate.unwrap();

    let mut metrics: HashMap<String, f64> = HashMap::new();
    if ctx.config.with_diagnosis {
      metrics.insert(String::from("billRate"), rate);
    }
    let rating = self.get_weight() * rate;
    log::debug!("Completed {} with rating {}", self.get_name(), rating);
    RatingResult {
      rating,
      metrics,
    }
  }
}
