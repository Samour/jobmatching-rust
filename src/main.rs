mod domain;
mod dto;
mod repositories;
mod routes;
mod services;

use repositories::rest::RestRepositoryImpl;
use services::config::{ConfigService, FileConfigService};
use services::job_match::JobMatchServiceImpl;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialisation
    let mut config_service = FileConfigService::new();
    config_service.load_config("resources/config.json").unwrap();
    let rest_repository = Arc::new(RestRepositoryImpl::new(config_service.get_config().base_url.clone()));
    let job_match_service = Arc::new(JobMatchServiceImpl::new(rest_repository));

    // Start server
    warp::serve(routes::route(job_match_service, Arc::new(config_service)))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
