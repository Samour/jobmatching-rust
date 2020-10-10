mod domain;
mod dto;
mod engine;
mod errors;
mod repositories;
mod routes;
mod services;

use domain::config::RatingWeights;
use engine::available_on_start_day::AvailableOnStartDay;
use engine::match_rating::MatchRating;
use log::LevelFilter;
use repositories::rest::RestRepositoryImpl;
use services::config::{ConfigService, FileConfigService};
use services::job_match::JobMatchServiceImpl;
use simple_logger::SimpleLogger;
use std::sync::Arc;

fn build_match_ratings(weights: &RatingWeights) -> Vec<Box<dyn MatchRating + Send + Sync>> {
    vec![Box::new(AvailableOnStartDay::new(
        weights.available_on_start_days,
    ))]
}

#[tokio::main]
async fn main() {
    // Initialisation
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .with_module_level("jobmatching_rust", LevelFilter::Trace)
        .init()
        .unwrap();
    let mut config_service = FileConfigService::new();
    config_service.load_config("resources/config.json").unwrap();
    let rest_repository = Arc::new(RestRepositoryImpl::new(
        config_service.get_config().base_url.clone(),
    ));
    let job_match_service = Arc::new(JobMatchServiceImpl::new(
        Arc::new(build_match_ratings(&config_service.get_config().weights)),
        rest_repository,
    ));

    // Start server
    log::info!("Starting server on port {}", 3030);
    warp::serve(routes::route(job_match_service, Arc::new(config_service)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
