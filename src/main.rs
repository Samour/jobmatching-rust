mod domain;
mod dto;
mod engine;
mod errors;
mod repositories;
mod routes;
mod services;
mod collections;

use domain::config::RatingWeights;
use engine::available_on_start_day::AvailableOnStartDay;
use engine::can_drive::CanDrive;
use engine::distance::PythagorasDistanceEvaluator;
use engine::job_location::JobLocation;
use engine::job_positons::JobPositions;
use engine::match_rating::MatchRating;
use engine::pay_rate::PayRate;
use engine::required_certificates::HasRequiredCertificates;
use log::LevelFilter;
use repositories::rest::RestRepositoryImpl;
use services::config::{ConfigService, FileConfigService};
use services::job_match::JobMatchServiceImpl;
use services::rules::RulesServiceImpl;
use simple_logger::SimpleLogger;
use std::sync::Arc;

fn build_match_ratings(weights: &RatingWeights) -> Vec<Box<dyn MatchRating + Send + Sync>> {
    vec![
        Box::new(AvailableOnStartDay::new(weights.available_on_start_days)),
        Box::new(HasRequiredCertificates::new(weights.required_certificates)),
        Box::new(JobLocation::new(
            weights.job_location,
            Box::new(PythagorasDistanceEvaluator::new()),
        )),
        Box::new(PayRate::new(weights.pay_rate)),
        Box::new(CanDrive::new()),
        Box::new(JobPositions::new(weights.job_positions)),
    ]
}

#[tokio::main]
async fn main() {
    // Initialisation
    SimpleLogger::new()
        .with_level(LevelFilter::Warn)
        .with_module_level("jobmatching_rust", LevelFilter::Debug)
        .init()
        .unwrap();
    let mut config_service = FileConfigService::new();
    config_service.load_config("resources/config.json").unwrap();
    let rest_repository = Arc::new(RestRepositoryImpl::new(
        config_service.get_config().base_url.clone(),
    ));
    let rules_service = Arc::new(RulesServiceImpl::new(build_match_ratings(
        &config_service.get_config().weights,
    )));
    let job_match_service = Arc::new(JobMatchServiceImpl::new(
        rules_service.clone(),
        rest_repository,
    ));

    // Start server
    log::info!("Starting server on port {}", 3030);
    warp::serve(routes::route(
        rules_service,
        job_match_service,
        Arc::new(config_service),
    ))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
