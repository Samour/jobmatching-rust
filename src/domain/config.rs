use serde::Deserialize;

#[derive(Deserialize)]
pub struct RatingWeights {
  pub available_on_start_days: f64,
  pub required_certificates: f64,
  pub job_location: f64,
  pub pay_rate: f64,
  pub job_positions: f64,
}

#[derive(Deserialize)]
pub struct Config {
  pub app_name: String,
  pub jobs_to_return: u32,
  pub workers_to_return: u32,
  pub base_url: String,
  pub weights: RatingWeights,
}
