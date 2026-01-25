-- Create workout tables

-- workouts: A training session
CREATE TABLE workouts (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    workout_date    DATE NOT NULL,
    notes           TEXT,
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      BIGINT NOT NULL,
    updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      BIGINT,
    deleted_at      DATETIME,
    deleted_by      BIGINT,
    CONSTRAINT fk_workout_user FOREIGN KEY (created_by) REFERENCES accounts(id)
);

-- workout_exercises: Links exercises to workouts
CREATE TABLE workout_exercises (
    id              BIGINT AUTO_INCREMENT PRIMARY KEY,
    workout_id      BIGINT NOT NULL,
    exercise_id     BIGINT NOT NULL,
    position        INT NOT NULL DEFAULT 0,
    notes           TEXT,
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by      BIGINT NOT NULL,
    updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by      BIGINT,
    deleted_at      DATETIME,
    deleted_by      BIGINT,
    CONSTRAINT fk_we_workout FOREIGN KEY (workout_id) REFERENCES workouts(id),
    CONSTRAINT fk_we_exercise FOREIGN KEY (exercise_id) REFERENCES exercises(id)
);

-- exercise_sets: Individual set records
CREATE TABLE exercise_sets (
    id                   BIGINT AUTO_INCREMENT PRIMARY KEY,
    workout_exercise_id  BIGINT NOT NULL,
    position             INT NOT NULL DEFAULT 0,
    set_type             ENUM('weight', 'timed', 'distance') NOT NULL DEFAULT 'weight',
    reps                 INT,
    weight               DECIMAL(10,2),
    duration_seconds     INT,
    distance             DECIMAL(10,2),
    speed                DECIMAL(10,2),
    notes                TEXT,
    created_at           DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by           BIGINT NOT NULL,
    updated_at           DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by           BIGINT,
    deleted_at           DATETIME,
    deleted_by           BIGINT,
    CONSTRAINT fk_set_we FOREIGN KEY (workout_exercise_id) REFERENCES workout_exercises(id)
);

-- Index for common queries
CREATE INDEX idx_workouts_user_date ON workouts(created_by, workout_date);
CREATE INDEX idx_workout_exercises_workout ON workout_exercises(workout_id);
CREATE INDEX idx_exercise_sets_workout_exercise ON exercise_sets(workout_exercise_id);
