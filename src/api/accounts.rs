// crates.io
use actix_web::{
    web, http::header, HttpResponse,
    HttpRequest
};
use actix_session::{Session};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use rusqlite::{params, Connection};
use handlebars::Handlebars;

// Page information fed to Handlebars
#[derive(Serialize)]
struct Context {
    logged_in: u32
}

// REQUIRES: GET method
// MODIFIES: n/a
// EFFECTS: Render login page
//          If logged in, redirect to index page
//          with appropriate login/logout button
pub async fn render_login(
    req: HttpRequest,
    session: Session,
    hb: web::Data<Handlebars<'_>>
) -> HttpResponse {
    if let Some(_id) = session.get::<String>("login").unwrap() {
        let data = Context {
            logged_in: 1
        };
        let body = hb.render("index", &data).unwrap();

        // redirect
        return HttpResponse::Found().body(body)
    }

    //else open login page
    let file : NamedFile = NamedFile::open("./rustviz/html/login.html").unwrap();
    file.into_response(&req).unwrap()
}

// Used for deserializing login form data
// Matches key information in db
// Used for querying, inserting into db
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    token: String
}

// REQUIRES: POST method
// MODIFIES: Cookies
// EFFECTS: Creates cookie, logs user into site
pub async fn user_login(
    session: Session,
    form: web::Form<User>
) -> HttpResponse {
    // if already logged in, redirect to home
    if let Some(_id) = session.get::<String>("login").unwrap() {
        // redirect
        return HttpResponse::Found()
                .header(header::LOCATION, "/")
                .finish()
    }

    let conn = Connection::open("db/fp.sqlite3").unwrap();

    let mut stmt = conn.prepare(
        "SELECT * FROM users
        WHERE token=?"
    ).unwrap();

    let mut qry = None;
    
    if let Ok(rows) = stmt.query_map(&[&form.token], |row| {
        Ok(User {
            token: row.get_unwrap(0),
        })
    }) {
        // compile into Vec<User>
        let mut vec : Vec<User> = rows.map(|r| r.unwrap()).collect();
        qry = vec.pop();
    }

    // check valid credentials
    if let Some(usr) = qry {
        if &form.token == usr.token.as_str() {
            // remember user
            session.set("login", form.token.clone()).unwrap();
            // extend cookie every time user logs in
            session.renew();

            // StatusCode Found allows redirect
            return HttpResponse::Found()
                .header(header::LOCATION, "/rust-tutorial")
                .finish()
        }
    }
    
    // else user not found, token incorrect return 403
    HttpResponse::Unauthorized()
        .body("Incorrect token!")
}

// REQUIRES: POST method
// MODIFIES: Cookies
// EFFECTS: Remove cookies, log out current user
pub async fn user_logout(session: Session) -> HttpResponse {
    session.remove("login");
    HttpResponse::Found()
        .header(header::LOCATION, "/rust-tutorial")
        .finish()
}

// REQUIRES: GET method
// MODIFIES: n/a
// EFFECTS: Render create account page
//          If logged in, redirect to index page
pub async fn render_create(
    req: HttpRequest,
    session: Session
) -> HttpResponse {
    // if already logged in, redirect to home
    if let Some(_id) = session.get::<String>("login").unwrap() {
        // redirect
        return HttpResponse::Found()
            .header(header::LOCATION, "/")
            .finish()
    }

    //else open login page
    let file : NamedFile = NamedFile::open("./rustviz/html/create.html").unwrap();
    file.into_response(&req).unwrap()
}

// REQUIRES: POST method
// MODIFIES: Database
// EFFECTS: Create new user instance in db
//          Create session cookies, track user
pub async fn user_create(
    session: Session,
    form: web::Form<User>
) -> HttpResponse {
    // if already logged in, return error
    if let Some(_id) = session.get::<String>("login").unwrap() {
        return HttpResponse::Conflict()
                .body("Please log out!")
    }

    // trim whitespaces
    let token = form.token.trim().to_string();

    // check all fields are non-empty
    if token.is_empty() {
        return HttpResponse::BadRequest()
            .body("Form field must be non-empty!")
    }

    let conn = Connection::open("db/fp.sqlite3").unwrap();

    let mut stmt = conn.prepare(
        "SELECT * FROM users
        WHERE token"
    ).unwrap();

    let mut qry = None;
    if let Ok(rows) = stmt.query_map(&[&token], |row| {
        Ok(User {
            token: row.get_unwrap(0),
        })
    }) {
        // compile into Vec<User>
        let mut vec : Vec<User> = rows.map(|r| r.unwrap()).collect();
        qry = vec.pop();
    }

    // check if token was already used
    if let Some(_usr) = qry {
        return HttpResponse::Forbidden()
            .body("Token taken!")
    }

    // else
    // set cookies, remember user
    session.set("login", token.clone()).unwrap();

    // create user
    conn.execute(
        "INSERT INTO users (token)
        VALUES (?1)",
        params![&token]
    ).unwrap();

    return HttpResponse::Found()
        .header(header::LOCATION, "/")
        .finish()
}
