CREATE TABLE user (
    `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,

    `name` VARBINARY(32) NOT NULL,
    `password_hash` VARBINARY(254) NOT NULL,

    `created_at` DATETIME NOT NULL,
    `updated_at` DATETIME NOT NULL,

    PRIMARY KEY (id),
    UNIQUE KEY (name),

    KEY (created_at),
    KEY (updated_at)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;