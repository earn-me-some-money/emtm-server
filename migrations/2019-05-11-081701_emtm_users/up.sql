CREATE TABLE emtm_users (
  uid INTEGER PRIMARY KEY AUTO_INCREMENT,
  wechat_id VARCHAR(50) NOT NULL,
  phone VARCHAR(20) NOT NULL,
  personal_info TEXT NOT NULL ,
  username VARCHAR(20) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT false,
  tokens INTEGER NOT NULL DEFAULT 0,
  UNIQUE (username),
  INDEX username_ind (username)
) CHARACTER SET utf8mb4;

CREATE TABLE emtm_students (
  uid INTEGER PRIMARY KEY,
  # University name
  school VARCHAR(100) NOT NULL,
  credit INTEGER NOT NULL DEFAULT 100,
  # Number of jobs
  accepted INTEGER NOT NULL DEFAULT 100,
  # Number of finished jobs
  finished INTEGER NOT NULL DEFAULT 0,
  # The code of the student's major
  major VARCHAR(20) NOT NULL,
  # Number of years the student has been in the university
  year INTEGER NOT NULL
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_students`
  ADD CONSTRAINT `emtm_students_uid` FOREIGN KEY (`uid`) REFERENCES `emtm_users` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;


CREATE TABLE emtm_cows (
  uid INTEGER PRIMARY KEY,
  company VARCHAR(100) NOT NULL
) CHARACTER SET utf8mb4;

ALTER TABLE `emtm_cows`
  ADD CONSTRAINT `emtm_cows_uid` FOREIGN KEY (`uid`) REFERENCES `emtm_users` (`uid`) ON DELETE CASCADE ON UPDATE CASCADE;
COMMIT;

