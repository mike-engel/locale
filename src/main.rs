#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate accept_language;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;

use rocket::request::{self, FromRequest, Request};
use rocket::outcome::Outcome::*;
use rocket_contrib::{Json, Template, Value};
use accept_language::parse;

struct AcceptLanguage {
    user_languages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexContext {
    language: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptLanguage {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let accept_language_header = request.headers().get_one("Accept-Language");

        match accept_language_header {
            Some(languages) => Success(AcceptLanguage { user_languages: parse(languages) }),
            None => Success(AcceptLanguage { user_languages: vec![] }),
        }
    }
}

#[get("/")]
fn index(accept_language: AcceptLanguage) -> Template {
    let context = IndexContext {
        language: accept_language
            .user_languages
            .first()
            .unwrap_or(&String::from("en"))
            .to_owned(),
    };

    Template::render("index", &context)
}

#[get("/api")]
fn api(accept_language: AcceptLanguage) -> Json<Value> {
    Json(json!(accept_language.user_languages))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, api])
        .attach(Template::fairing())
        .launch();
}
