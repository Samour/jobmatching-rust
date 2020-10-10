use warp::reject::Reject;

#[derive(Debug)]
pub struct BadRequestError {}

impl BadRequestError {
  pub fn new() -> BadRequestError {
    BadRequestError {}
  }
}

impl Reject for BadRequestError {}
