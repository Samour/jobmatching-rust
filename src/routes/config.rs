use crate::services::job_match::JobMatchService;
use serde::Serialize;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

#[derive(Serialize)]
struct RuleConfig {
  name: String,
  weight: f64,
}

pub fn route<JMS>(job_match: Arc<JMS>) -> BoxedFilter<(impl Reply,)>
where
  JMS: JobMatchService + Send + Sync + 'static,
{
  warp::path("rulesConfig")
    .and(warp::get())
    .map(move || warp::reply::json(&job_match.get_rule_configs()))
    .boxed()
}
