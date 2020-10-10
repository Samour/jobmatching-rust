use super::match_rating::MatchRating;
use crate::dto::{JobDto, WorkerDto};
use chrono::Datelike;

pub struct AvailableOnStartDay {
  weight_value: f64,
}

impl AvailableOnStartDay {
  pub fn new(weight_value: f64) -> AvailableOnStartDay {
    AvailableOnStartDay { weight_value }
  }
}

impl MatchRating for AvailableOnStartDay {
  fn determine_rating(&self, worker: &WorkerDto, job: &JobDto) -> f64 {
    let day_required_idx = job.start_date.weekday().num_days_from_monday() + 1;
    let has_start_day = worker
      .availability
      .iter()
      .filter(|d| match d {
        Some(day) => day.day_index == day_required_idx,
        None => false,
      })
      .count()
      > 0;

    if has_start_day {
      self.weight_value
    } else {
      0.0
    }
  }
}
