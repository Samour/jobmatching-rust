use crate::services::rules::RulesService;
use serde::Serialize;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

#[derive(Serialize)]
struct RuleConfig {
  name: String,
  weight: f64,
}

pub fn route<RS>(rules_service: Arc<RS>) -> BoxedFilter<(impl Reply,)>
where
  RS: RulesService + Send + Sync + 'static,
{
  warp::path("rulesConfig")
    .and(warp::get())
    .map(move || warp::reply::json(&rules_service.get_rule_configs()))
    .boxed()
}
