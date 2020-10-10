use warp::reject::Reject;

#[derive(Debug)]
pub struct ServerError {}

impl ServerError {
  pub fn new() -> ServerError {
    ServerError {}
  }
}

impl Reject for ServerError {}
