use accept_language::parse;
use http::StatusCode;
use now_lambda::{error::NowError, lambda, IntoResponse, Request, Response};
use serde_json::json;
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    eprintln!("Starting API request");
    let header = request.headers().get("Accept-Language");
    let parsed_headers = match header {
        Some(language_header) => parse(language_header.to_str().unwrap_or("")),
        None => vec![],
    };
    eprintln!("Building API response");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!(parsed_headers)
                .as_str()
                .expect("Unable to stringify JSON")
                .to_owned(),
        )
        .expect("Internal Server Error");

    eprintln!("About to send API response");
    Ok(response)
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
