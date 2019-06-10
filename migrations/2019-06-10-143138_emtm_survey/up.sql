CREATE TABLE emtm_survey 
(
    mid         INTEGER NOT NULL,
    qn_id       INTEGER PRIMARY KEY AUTO_INCREMENT

 ) CHARACTER SET utf8mb4;

 ALTER TABLE `emtm_survey`
    ADD CONSTRAINT `survey_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

CREATE table emtm_question
(
    qn_id       INTEGER NOT NULL,
    q_id        INTEGER PRIMARY KEY AUTO_INCREMENT,
    q_type      BOOLEAN NOT NULL,
    q_content   VARCHAR(200) NOT NULL,
    q_choice    VARCHAR(500) NOT NULL,

    INDEX (qn_id)
 ) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_question`
    ADD CONSTRAINT `question_naire_id_constraint_0` FOREIGN KEY (`qn_id`) REFERENCES `emtm_survey` (`qn_id`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

CREATE TABLE emtm_answer
(
    qn_id       INTEGER NOT NULL,
    qa_id       INTEGER PRIMARY KEY AUTO_INCREMENT,
    user_id     INTEGER NOT NULL,
    user_answer VARCHAR(500) NOT NULL,

    INDEX (qn_id)
 ) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_answer`
    ADD CONSTRAINT `question_naire_id_constraint` FOREIGN KEY (`qn_id`) REFERENCES `emtm_survey` (`qn_id`) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE `emtm_answer`
    ADD CONSTRAINT `student_id_constraint` FOREIGN KEY (`user_id`) REFERENCES `emtm_students` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;