-- Add created_at and created_by fields to exercise_categories and exercises tables

ALTER TABLE exercise_categories
ADD COLUMN `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP AFTER `name`,
ADD COLUMN `created_by` BIGINT AFTER `created_at`;

ALTER TABLE exercises
ADD COLUMN `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP AFTER `category_id`,
ADD COLUMN `created_by` BIGINT AFTER `created_at`;

-- Set created_by to 0 (system user) for existing categories
UPDATE exercise_categories SET created_by = 0 WHERE created_by IS NULL;

-- Set created_by to 0 (system user) for any existing exercises
UPDATE exercises SET created_by = 0 WHERE created_by IS NULL;
