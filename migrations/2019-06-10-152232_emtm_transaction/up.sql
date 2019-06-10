CREATE TABLE emtm_transaction
(
    mid                 INTEGER NOT NULL,
    t_id                INTEGER PRIMARY KEY AUTO_INCREMENT,
    is_accept           BOOLEAN NOT NULL,
    t_type              VARCHAR(100) NOT NULL,
    t_info              VARCHAR(300) NOT NULL,
    t_loss              INTEGER NOT NULL

 ) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_transaction`
    ADD CONSTRAINT `transaction_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

CREATE TABLE emtm_express
(
    mid                 INTEGER NOT NULL,
    e_id                INTEGER PRIMARY KEY AUTO_INCREMENT,
    is_accept           BOOLEAN NOT NULL,
    e_address           VARCHAR(100) NOT NULL,
    e_phone_number      VARCHAR(50) NOT NULL,
    e_pick_number       VARCHAR(20) NOT NULL,
    e_info              VARCHAR(200) NOT NULL

) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_express`
    ADD CONSTRAINT `express_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;