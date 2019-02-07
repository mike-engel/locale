use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;

fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
  let response = Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "application/json")
    .body("{\"status\":\"OK\"}")
    .expect("Internal Server Error");

  Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
  Ok(lambda!(handler))
}
