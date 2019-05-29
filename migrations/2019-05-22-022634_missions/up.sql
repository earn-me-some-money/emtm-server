CREATE TABLE emtm_missions
(
    mid              INTEGER PRIMARY KEY AUTO_INCREMENT,
    # The id of the user who post the mission
    poster_uid       INTEGER      NOT NULL,
    # The amount of reward per person of the mission
    bounty           INTEGER      NOT NULL,
    # The amount of tokens to be fined if
    # the student failed to finish the job
    risk             INTEGER      NOT NULL,
    # The name of the mission
    name             VARCHAR(200) NOT NULL,
    # The type code of the mission
    mission_type     TINYINT      NOT NULL,
    # The content and requirement of the mission
    content          TEXT         NOT NULL,
    # The time when the mission is posted
    post_time        DATETIME     NOT NULL,
    # The time when the mission is due
    deadline         DATETIME     NOT NULL,
    # The maximum number of participants
    max_participants INTEGER      NOT NULL,
    # The following restriction on participants are optional
    # The minimum grade of the participants
    min_grade        INTEGER DEFAULT NULL,
    # The maximum grade of the participants
    max_grade        INTEGER DEFAULT NULL,
    # The school the participant must come from,
    # set to null if no school restriction
    school           INTEGER DEFAULT NULL,
    # The minimum number of missions the student must have finished
    min_finished     INTEGER DEFAULT NULL,
    INDEX user_index (poster_uid, name),
    UNIQUE (poster_uid, name)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_missions`
    ADD CONSTRAINT `mission_cow_id_constra` FOREIGN KEY (`poster_uid`) REFERENCES `emtm_users` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE `emtm_missions`
    ADD CONSTRAINT `required_school_exist_constra` FOREIGN KEY (`school`) REFERENCES `school_zh` (`school_id`) ON DELETE RESTRICT ON UPDATE RESTRICT;
COMMIT;

CREATE TABLE emtm_participants
(
    mid         INTEGER,
    # The students' id
    student_uid INTEGER,
    # The current state of the student in the mission
    # 0 - Accepted(expired if over deadline)
    # 1 - finished
    # 2 - give up(cancel)
    state       TINYINT NOT NULL CHECK ( state in (0, 1, 2)),
    PRIMARY KEY (mid, student_uid)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_participants`
    ADD CONSTRAINT `mission_exists_constra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE `emtm_participants`
    ADD CONSTRAINT `mission_student_exists_constra` FOREIGN KEY (`student_uid`) REFERENCES `emtm_students` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;
