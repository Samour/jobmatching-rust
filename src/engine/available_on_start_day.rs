use super::config::EvaluationContext;
use super::match_rating::MatchRating;
use super::match_rating::RatingResult;
use chrono::Datelike;
use std::collections::HashMap;

pub struct AvailableOnStartDay {
  weight_value: f64,
}

impl AvailableOnStartDay {
  pub fn new(weight_value: f64) -> AvailableOnStartDay {
    AvailableOnStartDay { weight_value }
  }
}

impl MatchRating for AvailableOnStartDay {
  fn get_name(&self) -> &str {
    "AvailableOnStartDay"
  }

  fn get_weight(&self) -> f64 {
    self.weight_value
  }

  fn determine_rating(&self, ctx: &EvaluationContext) -> RatingResult {
    log::debug!(
      "Running rule {} for worker {} on job {}",
      self.get_name(),
      ctx.worker.user_id,
      ctx.job.job_id
    );
    let day_required_idx = ctx.job.start_date.weekday().num_days_from_monday() + 1;
    let has_start_day = ctx
      .worker
      .availability
      .iter()
      .filter(|d| match d {
        Some(day) => day.day_index == day_required_idx,
        None => false,
      })
      .count();
    let rating = if has_start_day > 0 {
      self.weight_value
    } else {
      0.0
    };
    let mut metrics: HashMap<String, f64> = HashMap::new();
    if ctx.config.with_diagnosis {
      metrics.insert(String::from("hasStartDay"), has_start_day as f64);
    }

    log::debug!("Rule {} completed; score: {}", self.get_name(), rating);
    RatingResult { rating, metrics }
  }
}
