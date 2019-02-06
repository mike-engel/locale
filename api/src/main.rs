use accept_language::parse;
use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use std::error::Error;
use serde_json::json;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let header = request.headers().get("Accept-Language");
    let parsed_headers = match header {
        Some(language_header) => parse(language_header.to_str().unwrap_or("")),
        None => vec![],
    };
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json!(parsed_headers))
        .expect("Internal Server Error");

    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
