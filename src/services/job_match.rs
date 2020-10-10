use crate::dto::JobDto;
use crate::repositories::rest::RestRepository;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait JobMatchService {
  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Box<dyn std::error::Error>>;
}

pub struct JobMatchServiceImpl {
  rest_repository: Arc<dyn RestRepository + Send + Sync>,
}

impl JobMatchServiceImpl {
  pub fn new(rest_repository: Arc<dyn RestRepository + Send + Sync>) -> JobMatchServiceImpl {
    JobMatchServiceImpl { rest_repository }
  }
}

#[async_trait]
impl JobMatchService for JobMatchServiceImpl {
  async fn find_best_jobs_for_worker(
    &self,
    worker_id: u32,
    job_limit: u32,
  ) -> Result<Vec<JobDto>, Box<dyn std::error::Error>> {
    Ok(self.rest_repository.find_all_jobs().await?[..(job_limit as usize)].to_vec())
  }
}
