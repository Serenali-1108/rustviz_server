PRAGMA foreign_keys = ON;

-- INITIALIZE USERS TABLE (with admins)
INSERT INTO users(token)
VALUES ('f6e035e8c2a2898938e59e5a361b9faddf4e68e');
INSERT INTO users(token)
VALUES ('59e5a361b9faddf4e68ef6e035e8c2a2898938e');

-- INITIALIZE QUESTIONS TABLE (with answers)
INSERT INTO questions (question_id, answer, filename, prompt, contains_fr)
VALUES (0, 0, 'vis_04_01_01', 'What qid is this (0)?', 1);

INSERT INTO questions (question_id, answer, filename, prompt, contains_fr)
VALUES (1, 0, 'vis_04_01_01', 'What qid is this (1)?', 1);

INSERT INTO questions (question_id, answer, filename, prompt, contains_fr)
VALUES (2, 0, 'vis_04_01_02', 'What qid is this (2)?', 1);

INSERT INTO questions (question_id, answer, filename, prompt, contains_fr)
VALUES (3, 0, 'vis_04_01_03', 'What qid is this (3)?', 1);

INSERT INTO questions (question_id, answer, filename, prompt, contains_fr)
VALUES (4, 0, 'vis_04_01_04', 'What qid is this (4)?', 1);

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (5, 0, 'vis_04_01_05', 'What qid is this (5)?');

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (6, 0, 'vis_04_01_06', 'What qid is this (6)?');

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (7, 0, 'vis_04_01_07', 'What qid is this (7)?');

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (8, 0, 'vis_04_01_08', 'What qid is this (8)?');

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (9, 0, 'vis_04_01_09', 'What qid is this (9)?');

INSERT INTO questions (question_id, answer, filename, prompt)
VALUES (10, 0, 'vis_04_01_10', 'What qid is this (#)?');

-- INITIALIZE MULTIPLE CHOICE TABLE (with answers)
INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (0, 0, 'option 0');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (0, 1, 'option 1');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (0, 2, 'option 2');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (0, 3, 'option 3');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (1, 0, 'option 0');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (1, 1, 'option 1');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (1, 2, 'option 2');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (1, 3, 'option 3');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (2, 0, 'option 0');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (2, 1, 'option 1');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (2, 2, 'option 2');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (2, 3, 'option 3');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (3, 0, 'option 0');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (3, 1, 'option 1');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (4, 0, 'option 4');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (5, 0, 'option 5');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (6, 0, 'option 6');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (7, 0, 'option 7');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (8, 0, 'option 8');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (9, 0, 'option 9');

INSERT INTO choices (question_id, ans_id, choice_text)
VALUES (10, 0, 'option 10');
