use accept_language::parse;
use now_lambda::{error::NowError, lambda, IntoResponse, Request};
use serde_json::json;
use std::error::Error;

fn handler(request: Request) -> Result<impl IntoResponse, NowError> {
    let header = request.headers().get("Accept-Language");
    let parsed_headers = match header {
        Some(language_header) => parse(language_header.to_str().unwrap_or("")),
        None => vec![],
    };

    Ok(json!(parsed_headers))
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}
