-- Add up migration script here
CREATE TABLE accounts
(
    `id`             BIGINT AUTO_INCREMENT,

    `name`           VARCHAR(128)     NOT NULL,
    `email`          VARCHAR(128)     NOT NULL,
    `password_hash`  VARCHAR(255)     NOT NULL,

    `created_at`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `created_by`     BIGINT       NOT NULL,

    `updated_at`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_by`     BIGINT       NOT NULL,

    `deleted_at`     DATETIME,
    `deleted_by`     BIGINT,
    `active`         TINYINT(1)   NOT NULL DEFAULT 1,

    PRIMARY KEY (id),

    INDEX account_email(`name`)
);
