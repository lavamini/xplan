CREATE TABLE user (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(32) NOT NULL,
    `password_hash` VARCHAR(255) NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    PRIMARY KEY (id),
    UNIQUE KEY (username),
    KEY (created_at),
    KEY (updated_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE employee (
    `emp_no` INT NOT NULL,
    `birth_date` DATE  NOT NULL,
    `first_name` VARCHAR(14) NOT NULL,
    `last_name` VARCHAR(16) NOT NULL,
    `gender` ENUM ('M','F') NOT NULL,
    `hire_date` DATE NOT NULL,

    PRIMARY KEY (emp_no),
    KEY (hire_date)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
