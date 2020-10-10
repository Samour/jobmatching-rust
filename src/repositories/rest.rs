use crate::dto::JobDto;
use async_trait::async_trait;

#[async_trait]
pub trait RestRepository {
  async fn find_all_jobs(&self) -> Result<Vec<JobDto>, Box<dyn std::error::Error>>;
}

pub struct RestRepositoryImpl {
  base_url: String,
}

impl RestRepositoryImpl {
  pub fn new(base_url: String) -> RestRepositoryImpl {
    RestRepositoryImpl { base_url }
  }
}

#[async_trait]
impl RestRepository for RestRepositoryImpl {
  async fn find_all_jobs(&self) -> Result<Vec<JobDto>, Box<dyn std::error::Error>> {
    let path = format!("{}/{}", self.base_url, "jobs");
    let jobs = reqwest::get(&path).await?.json::<Vec<JobDto>>().await?;

    Ok(jobs)
  }
}
