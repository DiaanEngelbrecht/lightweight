-- Drop workout tables in reverse order (due to foreign key constraints)
DROP TABLE IF EXISTS exercise_sets;
DROP TABLE IF EXISTS workout_exercises;
DROP TABLE IF EXISTS workouts;
