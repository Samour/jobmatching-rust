use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct NameDto {
  first: String,
  last: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeographicAreaDto {
  pub latitude: String,
  pub longitude: String,
  #[serde(rename = "maxJobDistance")]
  pub max_job_distance: f64,
  pub unit: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DayDto {
  pub title: String,
  #[serde(rename = "dayIndex")]
  pub day_index: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorkerDto {
  pub guid: String,
  #[serde(rename = "userId")]
  pub user_id: u32,
  #[serde(rename = "isActive")]
  pub active: bool,
  pub phone: String,
  pub email: String,
  pub name: NameDto,
  pub age: u32,
  pub rating: u32,
  pub certificates: Vec<Option<String>>,
  pub skills: Vec<String>,
  #[serde(rename = "jobSearchAddress")]
  pub job_search_address: GeographicAreaDto,
  pub transportation: String,
  #[serde(rename = "hasDriversLicense")]
  pub has_drivers_license: bool,
  pub availability: Vec<Option<DayDto>>,
}
