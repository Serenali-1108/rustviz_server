PRAGMA journal_mode=WAL;
PRAGMA foreign_keys = ON;

CREATE TABLE users (
  token VARCHAR(20) NOT NULL, -- username, at most 20 chars, primary key
  curr_ques INTEGER DEFAULT 0,
	created DATETIME DEFAULT CURRENT_TIMESTAMP, -- created, DATETIME type, automatically set by SQL engine to current date/time
	PRIMARY KEY (token)
);

/* QUIZ RELATED */

CREATE TABLE responses (
  token INTEGER NOT NULL,
  question_id INTEGER NOT NULL,
  answer INTEGER NOT NULL DEFAULT -2, -- default is -2 (not submited)
  time_elapsed INTEGER NOT NULL DEFAULT 0, -- time in ms
  hover_time INTEGER NOT NULL DEFAULT 0, -- time spent hovering in ms
  free_response TEXT NOT NULL DEFAULT '',
  PRIMARY KEY(token, question_id)
  FOREIGN KEY (token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE questions (
  question_id INTEGER NOT NULL,
  answer INTEGER NOT NULL,
  filename VARCHAR(20) NOT NULL,
  prompt VARCHAR(256) NOT NULL,
  contains_fr INTEGER NOT NULL DEFAULT 0, -- Boolean: contains free response, default is no
  PRIMARY KEY (question_id)
);

CREATE TABLE choices (
  question_id INTEGER NOT NULL,
  ans_id INTEGER NOT NULL,
  choice_text VARCHAR(256) NOT NULL,
  PRIMARY KEY (question_id, ans_id)
  FOREIGN KEY (question_id) REFERENCES questions(question_id) ON UPDATE CASCADE ON DELETE CASCADE
);

/* ASSIGNMENT-RELATED */

CREATE TABLE edit_states (
  token INTEGER NOT NULL,
  edit_state TEXT NOT NULL DEFAULT '',
  FOREIGN KEY (token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE scores (
  token INTEGER NOT NULL,
  problem_id INTEGER NOT NULL, /* 0 - 5; 6 problems total*/
  score INTEGER NOT NULL DEFAULT 0, -- default is 0, if answer compiled successfully then 1
  PRIMARY KEY(token, problem_id)
  FOREIGN KEY (token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE user_hover (
  token INTEGER NOT NULL,
  svg_name VARCHAR(256) NOT NULL,
  hover_item VARCHAR(256) NOT NULL,
  hover_times INTEGER NOT NULL,
  PRIMARY KEY(token, svg_name, hover_item),
  FOREIGN KEY(token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
);

-- CREATE TABLE user_page (
--   token INTEGER NOT NULL,
--   page_item VARCHAR(256) NOT NULL,
--   page_hover INTEGER NOT NULL,
--   page_visit INTEGER NOT NULL,
--   PRIMARY KEY(token, page_item),
--   FOREIGN KEY(token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
-- );

CREATE TABLE user_page (
  ID INTEGER PRIMARY KEY AUTOINCREMENT,
  token INTEGER NOT NULL,
  page_item VARCHAR(256) NOT NULL,
  page_hover INTEGER NOT NULL,
  FOREIGN KEY(token) REFERENCES users(token) ON UPDATE CASCADE ON DELETE CASCADE
);