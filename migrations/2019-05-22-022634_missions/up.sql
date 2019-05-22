CREATE TABLE emtm_missions
(
    mid       INTEGER PRIMARY KEY AUTO_INCREMENT,
    # The id of the cow who post the mission
    cow_uid   INTEGER      NOT NULL,
    # The amount of reward per person of the mission
    bounty    INTEGER      NOT NULL,
    # The amount of tokens to be fined if
    # the student failed to finish the job
    risk      INTEGER      NOT NULL,
    # The name of the mission
    name      VARCHAR(200) NOT NULL,
    # The content and requirement of the mission
    content   TEXT         NOT NULL,
    # The time when the mission is posted
    post_time DATETIME     NOT NULL,
    # The time when the mission is due
    deadline  DATETIME     NOT NULL,
    INDEX cow_index (cow_uid)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_missions`
    ADD CONSTRAINT `mission_cow_id_constra` FOREIGN KEY (`cow_uid`) REFERENCES `emtm_cows` (`uid`) ON DELETE RESTRICT ON UPDATE RESTRICT;
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
