CREATE TABLE emtm_users (
  uid INTEGER PRIMARY KEY AUTO_INCREMENT,
  wechat_id VARCHAR(50) NOT NULL,
  phone VARCHAR(20) NOT NULL,
  personal_info TEXT NOT NULL,
  username VARCHAR(20) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT false,
  tokens INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE emtm_students (
  uid INTEGER PRIMARY KEY,
  school VARCHAR(100)
);

ALTER TABLE `emtm_students`
  ADD CONSTRAINT `emtm_students_uid` FOREIGN KEY (`uid`) REFERENCES `emtm_users` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;


CREATE TABLE emtm_cows (
  uid INTEGER PRIMARY KEY,
  company VARCHAR(100)
);

ALTER TABLE `emtm_cows`
  ADD CONSTRAINT `emtm_cows_uid` FOREIGN KEY (`uid`) REFERENCES `emtm_users` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

