use super::config::EvaluationContext;
use super::match_rating::{MatchRating, RatingResult};
use std::collections::HashMap;

pub struct CanDrive {}

impl CanDrive {
  pub fn new() -> CanDrive {
    CanDrive {}
  }
}

impl MatchRating for CanDrive {
  fn get_name(&self) -> &str {
    "CanDrive"
  }

  fn get_weight(&self) -> f64 {
    1.0
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
        String::from("driversLicenseRequired"),
        if ctx.job.driver_license_required {
          1.0
        } else {
          0.0
        },
      );
      metrics.insert(
        String::from("hasDriversLicense"),
        if ctx.worker.has_drivers_license {
          1.0
        } else {
          0.0
        },
      );
    }

    if ctx.job.driver_license_required && !ctx.worker.has_drivers_license {
      log::debug!(
        "Worker does not satisfy drivers licence requirement; {} rule failure",
        self.get_name()
      );
      RatingResult {
        rating: -1.0,
        metrics,
      }
    } else {
      log::debug!(
        "Worker meets drivers license requirement: {} rule passes",
        self.get_name()
      );
      RatingResult {
        rating: 0.0,
        metrics,
      }
    }
  }
}
