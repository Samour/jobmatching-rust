use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Serialize, Deserialize, Clone)]
pub struct GeographicLocationDto {
  pub latitude: String,
  pub longitude: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JobDto {
  #[serde(rename = "jobId")]
  pub job_id: u32,
  pub guid: String,
  pub location: GeographicLocationDto,
  #[serde(rename = "billRate")]
  pub bill_rate: String,
  #[serde(rename = "workersRequired")]
  pub workers_required: u32,
  #[serde(rename = "driverLicenseRequired")]
  pub driver_license_required: bool,
  #[serde(rename = "requiredCertificates")]
  pub required_certificates: Vec<String>,
  #[serde(rename = "startDate")]
  pub start_date: DateTime<FixedOffset>,
  pub about: String,
  pub company: String,
}
