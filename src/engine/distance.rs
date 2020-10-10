use crate::dto::{GeographicAreaDto, GeographicLocationDto};
use std::f64::consts::PI;
use std::num::ParseFloatError;

const LAT_TO_KM: f64 = 110.574;
const LONG_TO_KM_C: f64 = 11.320;

pub trait GeographicDistanceEvaluator {
  fn determine_distance(
    &self,
    location1: &GeographicAreaDto,
    location2: &GeographicLocationDto,
  ) -> Result<f64, ParseFloatError>;
}

pub struct PythagorasDistanceEvaluator {}

impl PythagorasDistanceEvaluator {
  pub fn new() -> PythagorasDistanceEvaluator {
    PythagorasDistanceEvaluator {}
  }

  fn lat_to_km(&self, lat: f64) -> f64 {
    LAT_TO_KM * lat
  }

  fn long_to_km(&self, lat: f64, long: f64) -> f64 {
    LONG_TO_KM_C * (lat * 2.0 * PI / 360.0).cos() * long
  }
}

impl GeographicDistanceEvaluator for PythagorasDistanceEvaluator {
  fn determine_distance(
    &self,
    location1: &GeographicAreaDto,
    location2: &GeographicLocationDto,
  ) -> Result<f64, ParseFloatError> {
    let lat_diff =
      self.lat_to_km(location1.latitude.parse::<f64>()? - location2.latitude.parse::<f64>()?);
    let long_diff = self.long_to_km(
      (location1.latitude.parse::<f64>()? - location2.latitude.parse::<f64>()?).abs() / 2.0,
      location1.longitude.parse::<f64>()? - location2.longitude.parse::<f64>()?,
    );

    Ok((lat_diff.powi(2) + long_diff.powi(2)).sqrt())
  }
}
