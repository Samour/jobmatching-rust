use crate::dto::{JobDto, WorkerDto};
use crate::errors::server::ServerError;
use async_trait::async_trait;
use warp::reject::Rejection;

#[async_trait]
pub trait RestRepository {
  async fn find_all_workers(&self) -> std::result::Result<Vec<WorkerDto>, Rejection>;
  async fn find_worker_by_id(&self, worker_id: u32) -> Result<Option<WorkerDto>, Rejection>;
  async fn find_all_jobs(&self) -> Result<Vec<JobDto>, Rejection>;
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
  async fn find_all_workers(&self) -> Result<Vec<WorkerDto>, Rejection> {
    let path = format!("{}/{}", self.base_url, "workers");
    let response = reqwest::get(&path).await;
    if let Err(e) = response {
      println!("Error retrieving data from Rest resource {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }
    let workers = response.unwrap().json().await;

    match workers {
      Ok(w) => Ok(w),
      Err(e) => {
        println!("Error parsing worker data {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }

  async fn find_worker_by_id(&self, worker_id: u32) -> Result<Option<WorkerDto>, Rejection> {
    let matching_workers: Vec<WorkerDto> = self
      .find_all_workers()
      .await?
      .iter()
      .filter(|w| w.user_id == worker_id)
      .cloned()
      .collect();

    if matching_workers.len() > 0 {
      Ok(Some(matching_workers[0].clone()))
    } else {
      Ok(None)
    }
  }

  async fn find_all_jobs(&self) -> Result<Vec<JobDto>, Rejection> {
    let path = format!("{}/{}", self.base_url, "jobs");
    let response = reqwest::get(&path).await;
    if let Err(e) = response {
      println!("Error retrieving data from Rest resource {:?}", e);
      return Err(warp::reject::custom(ServerError::new()));
    }

    let jobs = response.unwrap().json().await;
    match jobs {
      Ok(j) => Ok(j),
      Err(e) => {
        println!("Error parsing jobs data {:?}", e);
        Err(warp::reject::custom(ServerError::new()))
      }
    }
  }
}
