use super::config::EvaluationContext;
use super::distance::GeographicDistanceEvaluator;
use super::match_rating::{MatchRating, RatingResult};
use std::collections::HashMap;

pub struct JobLocation {
  weight_value: f64,
  distance_evaluator: Box<dyn GeographicDistanceEvaluator + Send + Sync>,
}

impl JobLocation {
  pub fn new(
    weight: f64,
    distance_evaluator: Box<dyn GeographicDistanceEvaluator + Send + Sync>,
  ) -> JobLocation {
    JobLocation {
      weight_value: weight,
      distance_evaluator,
    }
  }
}

impl MatchRating for JobLocation {
  fn get_name(&self) -> &str {
    "JobLocation"
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
    if ctx.worker.job_search_address.unit != "km" {
      log::error!(
        "Worker {} has a job search location using '{}'; we only support 'km' at the moment",
        ctx.worker.user_id,
        ctx.worker.job_search_address.unit
      );
      return RatingResult {
        rating: -1.0,
        metrics: HashMap::new(),
      };
    }

    let distance = self
      .distance_evaluator
      .determine_distance(&ctx.worker.job_search_address, &ctx.job.location);
    if let Err(e) = distance {
      log::error!("Error when trying to calculate job site distance {:?}", e);
      return RatingResult {
        rating: -1.0,
        metrics: HashMap::new(),
      };
    }
    let distance = distance.unwrap();

    let mut metrics: HashMap<String, f64> = HashMap::new();
    if ctx.config.with_diagnosis {
      metrics.insert(String::from("distance"), distance);
      metrics.insert(
        String::from("maxJobDistance"),
        ctx.worker.job_search_address.max_job_distance,
      );
    }
    if distance > ctx.worker.job_search_address.max_job_distance {
      log::debug!("Job location is too far away");
      RatingResult {
        rating: -1.0,
        metrics,
      }
    } else {
      log::debug!("Job location is within search distance");
      let rating = self.get_weight() * (ctx.worker.job_search_address.max_job_distance - distance)
        / ctx.worker.job_search_address.max_job_distance;
      log::debug!("Rule {} completed with rating {}", self.get_name(), rating);
      RatingResult { rating, metrics }
    }
  }
}
