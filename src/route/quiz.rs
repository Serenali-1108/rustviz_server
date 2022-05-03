// crates.io
use actix_web::{web, HttpResponse};
use actix_session::{Session};
use serde::{Deserialize, Serialize};
use rusqlite::{NO_PARAMS, params, Connection};

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
pub struct UserResponse {
    ques_id: u8,
    ans_id: Option<u8>,
    free_response: Option<String>,
    time_elapsed_question: u64, // in milliseconds
    time_elapsed_hover: u64 // in milliseconds
}

// REQUIRES: valid user, POST method, valid ques_id, ans_id
// MODIFIES: results, user_ans
// EFFECTS: Record user's num of correct answers
//          Record user's response to each question
//          Increases user's num_correct by 1
//          if response submitted is correct
pub async fn record_response(
    session: Session,
    form: web::Json<UserResponse>
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

    // insert answer
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    // fetch total num of questions
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) AS num_questions
        FROM questions"
    ).unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    let num_questions : u8 = 
        if let Some(row) = rows.next().unwrap() { row.get_unwrap(0) }
        else { 0 };

    // check if form ques_id is valid
    if form.ques_id >= num_questions {
        return HttpResponse::BadRequest()
            .finish()
    }

    // if ans_id or free_response are None
    // only update time elapsed
    match (
        form.ques_id, form.ans_id,
        form.free_response.as_ref(),
        form.time_elapsed_question,
        form.time_elapsed_hover
    ) {
        (qid, None, _, time, hover_time) | (qid, _, None, time, hover_time) => {
            // check if already submitted
            let mut stmt = conn.prepare(
                "SELECT question_id FROM responses
                WHERE token = ?1 AND question_id = ?2"
            ).unwrap();

            if let Ok(mut rows) = stmt.query(&[&user, &qid.to_string()]) {
                // if question has not yet been submitted,
                //  initialize time_elapsed_question in db and update answer later
                if let None = rows.next().unwrap() {
                    // Note:
                    //      If answer = -2, user has not submitted answer
                    conn.execute(
                        "INSERT INTO responses (token, question_id)
                        VALUES (?1, ?2)",
                        params![&user, qid.to_string()]
                    ).unwrap();
                }
            };

            conn.execute(
                "UPDATE responses
                SET time_elapsed = time_elapsed + ?1,
                    hover_time = hover_time + ?2
                WHERE token = ?3 AND question_id = ?4",
                params![
                    time.to_string(),
                    hover_time.to_string(),
                    &user, qid.to_string()
                ]
            ).unwrap();
        },

        (qid, Some(aid), Some(free_res), time, hover_time) => {
            // fetch total num of choices
            let mut stmt = conn.prepare(
                "SELECT COUNT(*) AS num_choices
                FROM choices WHERE question_id=?"
            ).unwrap();
            let mut rows = stmt.query(params![qid]).unwrap();
            let num_choices : u8 = 
                if let Some(row) = rows.next().unwrap() { row.get_unwrap(0) }
                else { 0 };

            // check if form ans_id is valid
            if aid >= num_choices {
                return HttpResponse::BadRequest()
                    .finish()
            }

            // check if already submitted
            let mut stmt = conn.prepare(
                "SELECT question_id FROM responses
                WHERE token = ?1 AND question_id = ?2"
            ).unwrap();

            // NOTE:
            //      Ok to update free_response text even if
            //      question does not require free response field
            match stmt.query(&[&user, &qid.to_string()]) {
                Ok(mut rows) => {
                    // if question has been answered, update answer
                    if let Some(_row) = rows.next().unwrap() {
                        conn.execute(
                            "UPDATE responses
                            SET answer = ?1,
                                time_elapsed = time_elapsed + ?2,
                                hover_time = hover_time + ?3,
                                free_response = ?4
                            WHERE token = ?5 AND question_id = ?6",
                            params![
                                aid.to_string(), time.to_string(),
                                hover_time.to_string(), free_res,
                                &user, qid.to_string()
                            ]
                        ).unwrap();
                    }
                    // else create new instance and insert into table
                    else {
                        conn.execute(
                            "INSERT INTO responses (
                                token, question_id, answer,
                                time_elapsed,
                                hover_time, free_response
                            )
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                            params![
                                &user, qid.to_string(),
                                aid.to_string(), time.to_string(),
                                hover_time.to_string(), free_res
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
    };

        
    // update user progress
    conn.execute(
        "UPDATE users
        SET curr_ques = ?1
        WHERE token = ?2",
        params![(form.ques_id+1).to_string(), &user]
    ).unwrap();

    return HttpResponse::Created()
        .finish()
}

#[derive(Deserialize, Serialize)]
struct Question {
    qid: u8,
    filename: String,
    prompt: String,
    choices: Vec<Choice>,
    contains_free_response: bool
}

#[derive(Deserialize, Serialize)]
struct Choice {
    id: u8,
    text: String
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
pub async fn get_question(
    ques_id: web::Path<u8>
) -> HttpResponse {
    // convert to u8
    let ques_id = ques_id.into_inner();

    // open connection
    let conn = Connection::open("db/fp.sqlite3").unwrap();

    // fetch question filename and prompt
    let mut stmt = conn.prepare(
        "SELECT filename, prompt, contains_fr
        FROM questions WHERE question_id=?"
    ).unwrap();

    let mut filename = String::from("");
    let mut prompt = String::from("");
    let mut has_free_res = false;
    let mut rows = stmt.query(params![ques_id]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        filename = row.get_unwrap(0);
        prompt = row.get_unwrap(1);
        has_free_res = row.get_unwrap::<_, u8>(2) != 0; // convert INTEGER to bool
    }

    // fetch multiple choice answers
    let mut stmt = conn.prepare(
        "SELECT ans_id AS id, choice_text AS text
        FROM choices WHERE question_id=?"
    ).unwrap();

    // create vector of multiple choices
    let mut choice_vec : Vec<Choice> = Vec::new();
    if let Ok(rows) = stmt.query_map(params![ques_id], |row| {
        Ok(Choice {
            id: row.get_unwrap(0),
            text: row.get_unwrap(1)
        })
    }) {
        // compile into Vec<Choice>
        choice_vec = rows.map(|r| r.unwrap()).collect();
    }

    let q = Question {
        qid: ques_id,
        filename: filename,
        prompt: prompt,
        choices: choice_vec,
        contains_free_response: has_free_res
    };

    return HttpResponse::Ok()
        .json(q)
}

#[derive(Deserialize, Serialize)]
struct Quiz {
    total: u8,
    current: u8,
    saved_ans_vec: Vec<i8>,
    saved_free_res: Vec<String>,
    url: String
}

// REQUIRES: logged in user
// MODIFIES: n/a
// EFFECTS: Return current question id and uri,
//          total nums of questions
pub async fn init_quiz(
    session: Session
) -> HttpResponse {
    // check for login
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

    // fetch quiz status
    let mut stmt = conn.prepare(
        "SELECT curr_ques
        FROM users WHERE token=?"
    ).unwrap();
    let mut rows = stmt.query(params![&user]).unwrap();

    // Requires: user to be registered in db
    let mut last_ques = 0;
    if let Some(row) = rows.next().unwrap() {
        last_ques = row.get_unwrap(0);
    }

    // fetch total num of questions
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) AS num_questions
        FROM questions"
    ).unwrap();
    let mut rows = stmt.query(NO_PARAMS).unwrap();
    let num_questions : u8 = if let Some(row) = rows.next().unwrap() {
        row.get_unwrap(0)
    }
    else {
        0
    };

    // prepare vector with current answers
    let mut ans_vec : Vec<i8> = vec![-2; num_questions.into()]; // -2 if unanswered
    let mut fr_vec : Vec<String> = vec!["".to_string(); num_questions.into()]; // "" if unanswered
    // fetch current answers from db
    let mut stmt = conn.prepare(
        "SELECT question_id, answer, free_response
        FROM responses WHERE token=?"
    ).unwrap();
    let mut rows = stmt.query(params![&user]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let q_id : i8 = row.get_unwrap(0);
        let a_id : i8 = row.get_unwrap(1);
        let fr_text : String = row.get_unwrap(2);
        if let Some(elt) = ans_vec.get_mut(q_id as usize) {
            *elt = a_id;
        }
        if let Some(elt) = fr_vec.get_mut(q_id as usize) {
            *elt = fr_text;
        }
    }

    let q = Quiz {
        total: num_questions,
        current: last_ques,
        saved_ans_vec: ans_vec,
        saved_free_res: fr_vec,
        url: format!("/question/{}/", last_ques)
    };

    return HttpResponse::Ok()
        .json(q)
}
