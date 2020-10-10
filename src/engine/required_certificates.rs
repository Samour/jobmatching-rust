use super::match_rating::{MatchRating, RatingResult};
use crate::dto::{JobDto, WorkerDto};
use std::collections::{HashMap, HashSet};

const RATING_INCREMENT: f64 = 0.8;

pub struct HasRequiredCertificates {
  weight: f64,
}

impl HasRequiredCertificates {
  pub fn new(weight: f64) -> HasRequiredCertificates {
    HasRequiredCertificates { weight }
  }
}

impl MatchRating for HasRequiredCertificates {
  fn get_name(&self) -> &str {
    "HasRequiredCertificates"
  }

  fn get_weight(&self) -> f64 {
    self.weight
  }

  fn determine_rating(&self, worker: &WorkerDto, job: &JobDto) -> RatingResult {
    log::debug!(
      "Running rule {} for Worker {} and Job {}",
      self.get_name(),
      worker.user_id,
      job.job_id
    );
    let worker_certificates: HashSet<String> = worker
      .certificates
      .iter()
      .filter(|o| o.is_some())
      .map(|o| o.as_ref().unwrap().clone())
      .collect();
    let mut weighted_score: f64 = 0.0;
    let mut has_certs: i32 = 0;
    let mut missing_certs: i32 = 0;
    for required_cert in &job.required_certificates {
      if worker_certificates.contains(required_cert) {
        weighted_score += RATING_INCREMENT.powi(has_certs);
        has_certs += 1;
      } else {
        missing_certs += 1;
      }
    }

    let mut metrics: HashMap<String, f64> = HashMap::new();
    metrics.insert(String::from("ratingIncrement"), RATING_INCREMENT);
    metrics.insert(String::from("hasCertificates"), has_certs as f64);
    metrics.insert(String::from("missingCertificates"), missing_certs as f64);
    metrics.insert(String::from("weightedScore"), weighted_score);
    let rating = if missing_certs == 0 {
      weighted_score
    } else {
      -1.0
    };

    log::debug!("Rule {} completed; score {}", self.get_name(), rating);
    RatingResult { rating, metrics }
  }
}
