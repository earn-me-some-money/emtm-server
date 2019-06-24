CREATE TABLE emtm_trades
(
    mid            INTEGER PRIMARY KEY NOT NULL,
    # We don't care about this in the backend
    item_type      VARCHAR(100)        NOT NULL,
    # The description of the item
    item_info      VARCHAR(300)        NOT NULL,
    # The condition of the item, ranges from 0 to 5, less is better
    item_condition TINYINT             NOT NULL,
    # The address to exchange the item
    address        VARCHAR(200)        NOT NULL,

    INDEX (mid)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_trades`
    ADD CONSTRAINT `transaction_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

CREATE TABLE emtm_errands
(
    mid             INTEGER PRIMARY KEY NOT NULL,
    # The address to pick up item
    pickup_address  VARCHAR(100)        NOT NULL,
    # The phone number of the mission publisher
    phone_number    VARCHAR(50)         NOT NULL,
    # Optional field, the code for the item when picking up
    item_code       VARCHAR(20),
    # The address to deliver the item to
    deliver_address VARCHAR(100)        NOT NULL,
    # Other info like express company, etc.
    other_info      VARCHAR(200)        NOT NULL,

    INDEX (mid)
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_errands`
    ADD CONSTRAINT `express_mission_id_contra` FOREIGN KEY (`mid`) REFERENCES `emtm_missions` (`mid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;