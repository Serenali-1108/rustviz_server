use actix_web::{
    web, HttpResponse
};
use actix_session::{Session};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct hover_info {
    svg_name: String,
    hover_item: String
}

// REQUIRES: POST method
// MODIFIES: Number of hover for each user
// EFFECTS: record user hover
pub async fn user_hover(
    session: Session,
    info: web::Json<hover_info>,
) -> HttpResponse {
    // if not logged in, require authentication
    let user;
    if let None = session.get::<String>("login").unwrap() {
        return HttpResponse::Unauthorized()
            .body("Please log in!")
    }
    else {
        user = session.get::<String>("login").unwrap().unwrap();
    }

    // connect to sqlite
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    let mut stmt = conn.prepare(
        "SELECT * FROM user_hover
        WHERE token = ?1 AND svg_name = ?2 AND hover_item = ?3").unwrap();
    
    // update sql if no row found
    if let Ok(mut rows) = stmt.query(&[&user, &info.svg_name, &info.hover_item]) {
        if let None = rows.next().unwrap() {
            conn.execute(
                "INSERT INTO user_hover (token, svg_name, hover_item, hover_times)
                VALUES (?1, ?2, ?3, ?4)",
                params![&user, &info.svg_name, &info.hover_item, 0]
            ).unwrap();
        }
    };

    // update sql
    conn.execute(
        "UPDATE user_hover
        SET hover_times = hover_times + 1
        WHERE token = ?1 AND svg_name = ?2 AND hover_item = ?3",
        params![
            &user, &info.svg_name, &info.hover_item
        ]
    ).unwrap();

    println!("{:?}", info);
    return HttpResponse::Found().finish()
}

// REQUIRES: POST method
// MODIFIES: change user's SQL record
// EFFECTS: batch user's session result and push to SQL
// pub async fn user_close(
//     session: Session,
//     info: web::Json<Info>,
// ) -> HttpResponse {
//     // connect to sqlite
//     // let conn = Connection::open("db/fp.sqlite3").unwrap();
//     println!("{:?}", info);
//     return HttpResponse::Found().finish()
// }

#[derive(Deserialize, Debug)]
pub struct switch_info {
    directory: String,
    time_elpse: i64
}
// REQUIRES: POST method
// MODIFIES: SQL
// EFFECTS: record user's time spend on each chapter
// pub async fn user_switch(
//     session: Session,
//     info: web::Json<switch_info>,
// ) -> HttpResponse {
//     // if not logged in, require authentication
//     let user;
//     if let None = session.get::<String>("login").unwrap() {
//         return HttpResponse::Unauthorized()
//             .body("Please log in!")
//     }
//     else {
//         user = session.get::<String>("login").unwrap().unwrap();
//     }

//     // connect to sqlite
//     let conn = Connection::open("db/fp.sqlite3").unwrap();

//     let mut stmt = conn.prepare(
//         "SELECT * FROM user_page
//         WHERE token=?1 AND page_item=?2").unwrap();

//     if let Ok(mut rows) = stmt.query(&[&user, &info.directory]) {
//         if let None = rows.next().unwrap() {
//             conn.execute(
//                 "INSERT INTO user_page(token, page_item, page_hover, page_visit)
//                 VALUES(?1, ?2, ?3, ?4)", 
//                 params![&user, &info.directory, 0, 0]).unwrap();
//         }
//     }
//     conn.execute(
//         "UPDATE user_page
//         SET page_hover = page_hover + ?1, page_visit = page_visit + 1
//         WHERE token = ?2 AND page_item = ?3",
//         params![&info.time_elpse, &user, &info.directory]).unwrap();
    
//     // println!("-----------");
//     // println!("{:?}", info);
//     return HttpResponse::Found().finish()
// }

pub async fn user_switch(
    session: Session,
    info: web::Json<switch_info>,
) -> HttpResponse {
    // if not logged in, require authentication
    let user;
    if let None = session.get::<String>("login").unwrap() {
        return HttpResponse::Unauthorized()
            .body("Please log in!")
    }
    else {
        user = session.get::<String>("login").unwrap().unwrap();
    }

    // connect to sqlite
    let conn = Connection::open("db/fp.sqlite3").unwrap();
    
    conn.execute(
        "INSERT INTO user_page(token, page_item, page_hover)
        VALUES(?1, ?2, ?3)", 
        params![&user, &info.directory, &info.time_elpse]).unwrap();

    return HttpResponse::Found().finish()
}