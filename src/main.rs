#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate accept_language;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;

use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket_contrib::{JSON, Value};
use accept_language::parse;

struct AcceptLanguage {
    user_languages: Vec<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let accept_language_header = request.headers().get_one("Accept-Language");

        match accept_language_header {
            Some(languages) => Success(AcceptLanguage { user_languages: parse(languages) }),
            None => Success(AcceptLanguage { user_languages: vec![] })
        }
    }
}

#[get("/")]
fn index(accept_language: AcceptLanguage) -> JSON<Value> {
    JSON(json!(accept_language.user_languages))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
