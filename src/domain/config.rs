use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub app_name: String,
  pub jobs_to_return: u32,
  pub base_url: String,
}
