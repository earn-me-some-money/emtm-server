# Every single question
CREATE table emtm_questions
(
    mid      INTEGER      NOT NULL,
    # The place of the question in the questionnaire
    ordering INTEGER      NOT NULL,
    description  VARCHAR(400) NOT NULL,
    choices   VARBINARY(500) NOT NULL ,

    INDEX (mid),
    PRIMARY KEY (mid, ordering)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_questions`
    ADD CONSTRAINT `question_naire_id_constraint_0` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

# A user's answers to a whole questionnaire
CREATE TABLE emtm_answers
(
    mid         INTEGER         NOT NULL,
    user_id     INTEGER         NOT NULL,
    user_answer VARBINARY(3000) NOT NULL,

    INDEX (mid),
    PRIMARY KEY (mid, user_id)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_answers`
    ADD CONSTRAINT `question_naire_id_constraint` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE `emtm_answers`
    ADD CONSTRAINT `student_answer_id_constraint` FOREIGN KEY (`user_id`) REFERENCES `emtm_students` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;