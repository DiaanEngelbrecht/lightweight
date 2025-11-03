-- Revert created_at and created_by fields

ALTER TABLE exercise_categories
DROP COLUMN `created_by`,
DROP COLUMN `created_at`;

ALTER TABLE exercises
DROP COLUMN `created_by`,
DROP COLUMN `created_at`;
