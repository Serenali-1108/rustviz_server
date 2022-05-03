// crates.io
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use serde::{Deserialize, Serialize};
use rusqlite::{NO_PARAMS, params, Connection};
use std::process::{Command, Stdio};
use std::fs;
use execute::Execute;

const NUM_PROBLEMS: i32 = 6;

// URL form deserializes into this struct
// e.g.: POST "/submit" with some JSON body
//       will deserialize into:
//          Question {
//              ques_id: 0,
//              ans_id: 7,
//              free_response: "Does not compile because ...",
//              time_elapsed_question: 2000,
//              time_elapsed_hover: 12345
//          }
#[derive(Deserialize)]
pub struct Submission {
    to_grade: String,
    problem: i32,
    edit_state: String
}

#[derive(Deserialize, Serialize)]
pub struct SubmissionResult {
    correct: bool,
    output: String
}

pub async fn check(
    session: Session,
    form: web::Json<Submission>
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

    let mut command = Command::new("docker");

    command.arg("run");
    command.arg("rust-src");
    command.arg(&form.to_grade);

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());

    let output = command.execute_output().unwrap();
    
    let (error, output) = match output.status.code() {
        Some(0) => (false, String::from_utf8(output.stdout).unwrap()),
        _ => (true, String::from_utf8(output.stderr).unwrap())
    };

    let score = if error { 0 } else { 1 };

    // insert score
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    // check if form ques_id is valid
    if form.problem >= NUM_PROBLEMS {
        return HttpResponse::BadRequest()
            .finish()
    }

    // update user's score
    // check if already submitted
    let mut stmt = conn.prepare(
        "SELECT problem_id FROM scores
        WHERE token = ?1 AND problem_id = ?2"
    ).unwrap();

    match stmt.query(&[&user, &form.problem.to_string()]) {
        Ok(mut rows) => {
            // if question has been answered, update answer
            if let Some(_row) = rows.next().unwrap() {
                conn.execute(
                    "UPDATE scores
                    SET score = ?1
                    WHERE token = ?2 AND problem_id = ?3",
                    params![
                        &score.to_string(),
                        &user, &form.problem.to_string()
                    ]
                ).unwrap();
            }
            // else create new instance and insert into table
            else {
                conn.execute(
                    "INSERT INTO scores (
                        token, problem_id, score
                    )
                    VALUES (?1, ?2, ?3)",
                    params![
                        &user, form.problem.to_string(),
                        &score.to_string()
                    ]
                ).unwrap();
            }
        },
        _ => {
            return HttpResponse::Forbidden()
                .finish()
        }
    };
    
    // update the user's edit state
    let edit_state = &form.edit_state;
        conn.execute(
            "UPDATE edit_states
            SET edit_state = ?1
            WHERE token = ?2",
            params![
                &edit_state, &user
            ]
        ).unwrap();

    return HttpResponse::Ok()
    .json(
        SubmissionResult {
            correct: !error,
            output: output
        }
    );
}

pub async fn reset(
    session: Session,
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

    // insert score
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    // set score for all problems to 0
    for problem_id in 0..NUM_PROBLEMS {
        // update user's score
        // check if already submitted
        let mut stmt = conn.prepare(
            "SELECT problem_id FROM scores
            WHERE token = ?1 AND problem_id = ?2"
        ).unwrap();

        match stmt.query(&[&user, &problem_id.to_string()]) {
            Ok(mut rows) => {
                // if question has been answered, update answer
                if let Some(_row) = rows.next().unwrap() {
                    conn.execute(
                        "UPDATE scores
                        SET score = ?1
                        WHERE token = ?2 AND problem_id = ?3",
                        params![
                            0.to_string(),
                            &user, &problem_id.to_string()
                        ]
                    ).unwrap();
                }
                // else create new instance and insert into table
                else {
                    conn.execute(
                        "INSERT INTO scores (
                            token, problem_id, score
                        )
                        VALUES (?1, ?2, ?3)",
                        params![
                            &user, &problem_id.to_string(),
                            0.to_string()
                        ]
                    ).unwrap();
                }
            },
            _ => {
                return HttpResponse::Forbidden()
                    .finish()
            }
        };
    }
    
    // update the user's edit state
    let edit_state = fs::read_to_string("./rust-line-editor-ui/public/rust-src/src/problems.rs").unwrap();
        conn.execute(
            "UPDATE edit_states
            SET edit_state = ?1
            WHERE token = ?2",
            params![
                &edit_state, &user
            ]
        ).unwrap();

    return HttpResponse::Ok().finish();
}

#[derive(Deserialize, Serialize)]
struct EditState {
    answers: String,
    scores: Vec<i32>
}

// REQUIRES: GET method, valid question id
// MODIFIES: n/a
// EFFECTS: Returns qid_url_slug's information
//          e.g.: GET /question/0/ returns info for question 0
//              {
//                  qid: 0,
//                  filename: vis_04_01_01,
//                  prompt: 'What is the output of the function?',
//                  choices: [
//                      {id: 0, text:'Does not compile'},
//                      {id: 1, text:'5'},
//                      {id: 2, text:'15'}
//                  ],
//                  contains_free_response: true
//              }
pub async fn get_edit_state(
    session: Session
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
    
    // open connection
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    let answers: String;
    // fetch question filename and prompt
    let mut stmt = conn.prepare(
        "SELECT edit_state
        FROM edit_states WHERE token = ?"
    ).unwrap();

    // get the user's rust file
    let mut rows = stmt.query(params![&user]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        answers = row.get_unwrap(0);
    } else {
        answers = fs::read_to_string("./rust-line-editor-ui/public/rust-src/src/problems.rs").unwrap();
        conn.execute(
            "INSERT INTO edit_states (
                token, edit_state
            )
            VALUES (?1, ?2)",
            params![
                &user, &answers
            ]
        ).unwrap();
    }

    // get the user's scores
    let mut scores = Vec::new();
    for problem_id in 0..NUM_PROBLEMS {
        let mut stmt = conn.prepare(
            "SELECT score FROM scores
            WHERE token = ?1 AND problem_id = ?2"
        ).unwrap();

        match stmt.query(&[&user, &problem_id.to_string()]) {
            Ok(mut rows) => {
                if let Some(_row) = rows.next().unwrap() {
                    scores.push(_row.get_unwrap(0));
                }
                // else create new instance and insert into table
                else {
                    scores.push(0);
                    conn.execute(
                        "INSERT INTO scores (
                            token, problem_id, score
                        )
                        VALUES (?1, ?2, ?3)",
                        params![
                            &user, &problem_id.to_string(),
                            0.to_string()
                        ]
                    ).unwrap();
                }
            },
            _ => {
                return HttpResponse::Forbidden()
                    .finish()
            }
        };

    }

    return HttpResponse::Ok()
        .json(EditState {
            answers,
            scores
        });
}