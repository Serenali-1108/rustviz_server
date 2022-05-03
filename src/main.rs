// crates.io
use actix_web::{
    web, App, HttpServer,
    middleware, guard
};
use actix_files::Files as fs;
use actix_session::{CookieSession};
use handlebars::Handlebars;
use env_logger;
// local
use fplab_server::route::*;
use fplab_server::api::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // log requests to console
    std::env::set_var("RUST_LOG", "actix_web=error");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./rustviz/html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {  // move fp_data into closure
        App::new()
            // store data across app threads
            .app_data(handlebars_ref.clone())
            // normalize path: Merges multiple slashes into one, appends a trailing slash if one is not present.
            .wrap(middleware::NormalizePath)
            // log information
            .wrap(middleware::Logger::new(r#""%r" "%{Referer}i" "%{User-Agent}i""#))
            // create cookie based session middleware
            .wrap(
                CookieSession::signed(&[0; 32]) // <- create cookie based session middleware
                    .name("fplab_session")
                    .max_age(604800) // 1 week (seconds)
                    .secure(false)
            )
            // serve book files
            .service(
                fs::new("/book", "../rustviz-tutorial/book")
                    .index_file("index.html")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            // serve css, js files
            .service(
                fs::new("/css", "./rustviz/css")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            .service(
                fs::new("/js", "./rustviz/js")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            // serve quiz.html
            .service(
                fs::new("/fp", "./rustviz/quiz")
                    .index_file("quiz.html")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            // serve EECS 490 assignment FRONTEND
            .service(
                fs::new("/assignment", "./rust-line-editor-ui/build")
                    .index_file("index.html")
                    .redirect_to_slash_directory()
                    .use_guards(guard::Get())
            )
            .service(
                fs::new("/static", "./rust-line-editor-ui/build/static")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            .service(
                fs::new("/rust-src", "./rust-line-editor-ui/build/rust-src")
                    .redirect_to_slash_directory()
                    .disable_content_disposition()
                    .use_guards(guard::Get())
            )
            // serve EECS 490 assignment-related routes
            .route("/check", web::post().to(assignment::check))
            .route("/state", web::get().to(assignment::get_edit_state))
            .route("/reset", web::post().to(assignment::reset))
            // redirect route
            .route("/rust-tutorial", web::get().to(fp_web::index))
            // routes handlers
            .route("/accounts/login/", web::get().to(accounts::render_login))
            .route("/accounts/login/", web::post().to(accounts::user_login))
            .route("/accounts/logout/", web::post().to(accounts::user_logout))
            .route("/accounts/create/", web::get().to(accounts::render_create))
            .route("/accounts/create/", web::post().to(accounts::user_create))
            // detect user action
            .route("/action/hover", web::post().to(action::user_hover))
            // .route("/action/close", web::post().to(action::user_close))
            .route("/action/switch", web::post().to(action::user_switch))
            // quiz-related routes
            .route("/submit", web::post().to(quiz::record_response))
            .route("/question/{qid_url_slug}/", web::get().to(quiz::get_question))
            .route("/question/", web::get().to(quiz::init_quiz))
            // default to error page
            .default_service(
                web::route().to(fp_web::error)
            )
    })
    // .bind("141.212.113.125:8020")
    // .expect("Unable to bind to port 80!")
    .bind(("localhost", 8000))?
    .shutdown_timeout(10)
    .run()
    .await
}
