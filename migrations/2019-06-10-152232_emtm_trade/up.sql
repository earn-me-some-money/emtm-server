CREATE TABLE emtm_trades
(
    mid                 INTEGER NOT NULL,
    t_id                INTEGER PRIMARY KEY AUTO_INCREMENT,
    t_type              VARCHAR(100) NOT NULL,
    t_info              VARCHAR(300) NOT NULL,
    t_loss              INTEGER NOT NULL

 ) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_trades`
    ADD CONSTRAINT `transaction_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

CREATE TABLE emtm_errands
(
    mid                 INTEGER NOT NULL,
    e_id                INTEGER PRIMARY KEY AUTO_INCREMENT,
    e_address           VARCHAR(100) NOT NULL,
    e_phone_number      VARCHAR(50) NOT NULL,
    e_pick_number       VARCHAR(20) NOT NULL,
    e_info              VARCHAR(200) NOT NULL

) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_errands`
    ADD CONSTRAINT `express_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;