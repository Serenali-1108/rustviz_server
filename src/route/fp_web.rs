// crates.io
use actix_web::{
    web, HttpResponse
};
use actix_files::NamedFile;
use actix_session::{Session};
use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    logged_in: u32
}

// REQUIRES: GET method
// MODIFIES: Cookie
// EFFECTS: Render index page on
//          "fplab.eecs.umich.edu/rust-tutorial"
pub async fn index(
    session: Session,
    hb: web::Data<Handlebars<'_>>
) -> HttpResponse {
    if let Some(_id) = session.get::<String>("login").unwrap() {
        let data = Context {
            logged_in: 1
        };
        let body = hb.render("index", &data).unwrap();

        // redirect
        return HttpResponse::Ok().body(body)
    }
    else {
        let data = Context {
            logged_in: 0
        };
        let body = hb.render("index", &data).unwrap();

        // redirect
        return HttpResponse::Ok().body(body)
    }
}

// REQUIRES: GET method
// MODIFIES: n/a
// EFFECTS: Redirect all errors to custom error page
pub async fn error() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("./rustviz/html/error.html")
}
