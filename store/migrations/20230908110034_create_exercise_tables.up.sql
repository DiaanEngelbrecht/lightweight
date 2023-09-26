-- Add up migration script here

CREATE TABLE exercise_categories
(
    `id`             BIGINT AUTO_INCREMENT,

    `name`           VARCHAR(128)     NOT NULL,


    `updated_at`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_by`     BIGINT,

    `deleted_at`     DATETIME,
    `deleted_by`     BIGINT,
    `active`         TINYINT(1)   NOT NULL DEFAULT 1,

    PRIMARY KEY (id)
);


CREATE TABLE exercises
(
    `id`             BIGINT AUTO_INCREMENT,

    `name`           VARCHAR(128)     NOT NULL,
    `category_id`           BIGINT           NOT NULL,


    `updated_at`     DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_by`     BIGINT,

    `deleted_at`     DATETIME,
    `deleted_by`     BIGINT,
    `active`         TINYINT(1)   NOT NULL DEFAULT 1,

    PRIMARY KEY (id),
    CONSTRAINT e_category_ref FOREIGN KEY ec_ref_key (category_id) REFERENCES exercise_categories(id)
);
