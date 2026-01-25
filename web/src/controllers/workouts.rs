use super::AppError;
use crate::auth::authenticate_request;
use crate::config::Config;
use crate::protos::workouts::workouts_server::Workouts;
use crate::protos::workouts::{
    AddExerciseToWorkoutRequest, AddExerciseToWorkoutResponse, CreateSetRequest, CreateSetResponse,
    CreateWorkoutRequest, CreateWorkoutResponse, DeleteSetRequest, DeleteSetResponse,
    DeleteWorkoutRequest, DeleteWorkoutResponse, ExerciseSet as ProtoExerciseSet,
    GetTodaysWorkoutRequest, GetTodaysWorkoutResponse, GetWorkoutRequest, GetWorkoutResponse,
    ListWorkoutsRequest, ListWorkoutsResponse, RemoveExerciseFromWorkoutRequest,
    RemoveExerciseFromWorkoutResponse, UpdateSetRequest, UpdateSetResponse,
    UpdateWorkoutExerciseRequest, UpdateWorkoutExerciseResponse, Workout as ProtoWorkout,
    WorkoutExercise as ProtoWorkoutExercise, WorkoutSummary as ProtoWorkoutSummary,
};
use cali_core::store::get_conn;
use chrono::{NaiveDate, Utc};
use lightweight_store::repositories::workouts::contract::WorkoutsRepositoryContract;
use lightweight_store::repositories::workouts::implementation::WorkoutsRepository;
#[allow(unused_imports)]
use lightweight_store::repositories::workouts::models::WorkoutExerciseWithDetails;
use lightweight_store::repositories::workouts::models::{
    ExerciseSet, SetType, Workout, WorkoutExercise, WorkoutSummary,
};
use rust_decimal::Decimal;
use std::str::FromStr;
use tonic::async_trait;
use tonic::{Request, Response, Status};

cali_derive::controller!(WorkoutsController);

#[async_trait]
impl Workouts for WorkoutsController {
    async fn create_workout(
        &self,
        request: Request<CreateWorkoutRequest>,
    ) -> Result<Response<CreateWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let workout_date = NaiveDate::parse_from_str(&req_data.workout_date, "%Y-%m-%d")
            .map_err(|_| Status::invalid_argument("Invalid date format. Use YYYY-MM-DD"))?;

        let notes = if req_data.notes.is_empty() {
            None
        } else {
            Some(req_data.notes)
        };

        let new_workout = Workout::new(workout_date, notes, user_id);

        let mut conn = get_conn::<AppError>().await?;
        let workout_id =
            WorkoutsRepository::create_workout::<_, AppError>(&mut *conn, new_workout).await?;

        Ok(tonic::Response::new(CreateWorkoutResponse {
            success: true,
            result_code: 201,
            message: "Workout created successfully".to_string(),
            workout_id,
        }))
    }

    async fn get_workout(
        &self,
        request: Request<GetWorkoutRequest>,
    ) -> Result<Response<GetWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        let workout =
            WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, req_data.workout_id).await?;

        match workout {
            Some(w) => {
                // Check ownership
                if w.created_by != user_id {
                    return Err(Status::permission_denied(
                        "Not authorized to view this workout",
                    ));
                }

                // Get exercises and sets
                let exercises =
                    WorkoutsRepository::get_workout_exercises::<_, AppError>(&mut *conn, w.id)
                        .await?;

                let mut proto_exercises = Vec::new();
                for ex in exercises {
                    let sets = WorkoutsRepository::get_sets_for_workout_exercise::<_, AppError>(
                        &mut *conn, ex.id,
                    )
                    .await?;

                    proto_exercises.push(ProtoWorkoutExercise {
                        id: ex.id,
                        exercise_id: ex.exercise_id,
                        exercise_name: ex.exercise_name,
                        category_name: ex.category_name,
                        position: ex.position,
                        notes: ex.notes.unwrap_or_default(),
                        sets: sets.into_iter().map(|s| s.into()).collect(),
                    });
                }

                Ok(tonic::Response::new(GetWorkoutResponse {
                    success: true,
                    result_code: 200,
                    message: "Workout retrieved successfully".to_string(),
                    workout: Some(ProtoWorkout {
                        id: w.id,
                        workout_date: w.workout_date.format("%Y-%m-%d").to_string(),
                        notes: w.notes.unwrap_or_default(),
                        exercises: proto_exercises,
                    }),
                }))
            }
            None => Ok(tonic::Response::new(GetWorkoutResponse {
                success: false,
                result_code: 404,
                message: "Workout not found".to_string(),
                workout: None,
            })),
        }
    }

    async fn get_todays_workout(
        &self,
        request: Request<GetTodaysWorkoutRequest>,
    ) -> Result<Response<GetTodaysWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let today = Utc::now().date_naive();
        let mut conn = get_conn::<AppError>().await?;

        let workout =
            WorkoutsRepository::get_workout_by_date::<_, AppError>(&mut *conn, user_id, today)
                .await?;

        match workout {
            Some(w) => {
                // Get exercises and sets
                let exercises =
                    WorkoutsRepository::get_workout_exercises::<_, AppError>(&mut *conn, w.id)
                        .await?;

                let mut proto_exercises = Vec::new();
                for ex in exercises {
                    let sets = WorkoutsRepository::get_sets_for_workout_exercise::<_, AppError>(
                        &mut *conn, ex.id,
                    )
                    .await?;

                    proto_exercises.push(ProtoWorkoutExercise {
                        id: ex.id,
                        exercise_id: ex.exercise_id,
                        exercise_name: ex.exercise_name,
                        category_name: ex.category_name,
                        position: ex.position,
                        notes: ex.notes.unwrap_or_default(),
                        sets: sets.into_iter().map(|s| s.into()).collect(),
                    });
                }

                Ok(tonic::Response::new(GetTodaysWorkoutResponse {
                    success: true,
                    result_code: 200,
                    message: "Today's workout found".to_string(),
                    has_workout: true,
                    workout: Some(ProtoWorkout {
                        id: w.id,
                        workout_date: w.workout_date.format("%Y-%m-%d").to_string(),
                        notes: w.notes.unwrap_or_default(),
                        exercises: proto_exercises,
                    }),
                }))
            }
            None => Ok(tonic::Response::new(GetTodaysWorkoutResponse {
                success: true,
                result_code: 200,
                message: "No workout for today".to_string(),
                has_workout: false,
                workout: None,
            })),
        }
    }

    async fn list_workouts(
        &self,
        request: Request<ListWorkoutsRequest>,
    ) -> Result<Response<ListWorkoutsResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let limit = if req_data.limit <= 0 {
            20
        } else {
            req_data.limit
        };
        let offset = if req_data.offset < 0 {
            0
        } else {
            req_data.offset
        };

        let mut conn = get_conn::<AppError>().await?;
        let workouts =
            WorkoutsRepository::list_workouts::<_, AppError>(&mut *conn, user_id, limit, offset)
                .await?;

        Ok(tonic::Response::new(ListWorkoutsResponse {
            success: true,
            result_code: 200,
            message: "Workouts retrieved successfully".to_string(),
            workouts: workouts.into_iter().map(|w| w.into()).collect(),
        }))
    }

    async fn delete_workout(
        &self,
        request: Request<DeleteWorkoutRequest>,
    ) -> Result<Response<DeleteWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership
        let workout =
            WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, req_data.workout_id).await?;

        match workout {
            Some(w) if w.created_by == user_id => {
                WorkoutsRepository::delete_workout::<_, AppError>(
                    &mut *conn,
                    req_data.workout_id,
                    user_id,
                )
                .await?;

                Ok(tonic::Response::new(DeleteWorkoutResponse {
                    success: true,
                    result_code: 200,
                    message: "Workout deleted successfully".to_string(),
                }))
            }
            Some(_) => Err(Status::permission_denied(
                "Not authorized to delete this workout",
            )),
            None => Ok(tonic::Response::new(DeleteWorkoutResponse {
                success: false,
                result_code: 404,
                message: "Workout not found".to_string(),
            })),
        }
    }

    async fn add_exercise_to_workout(
        &self,
        request: Request<AddExerciseToWorkoutRequest>,
    ) -> Result<Response<AddExerciseToWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify workout ownership
        let workout =
            WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, req_data.workout_id).await?;

        match workout {
            Some(w) if w.created_by == user_id => {
                let position = WorkoutsRepository::get_next_exercise_position::<_, AppError>(
                    &mut *conn,
                    req_data.workout_id,
                )
                .await?;

                let notes = if req_data.notes.is_empty() {
                    None
                } else {
                    Some(req_data.notes)
                };

                let workout_exercise = WorkoutExercise::new(
                    req_data.workout_id,
                    req_data.exercise_id,
                    position,
                    notes,
                    user_id,
                );

                let workout_exercise_id =
                    WorkoutsRepository::add_exercise_to_workout::<_, AppError>(
                        &mut *conn,
                        workout_exercise,
                    )
                    .await?;

                Ok(tonic::Response::new(AddExerciseToWorkoutResponse {
                    success: true,
                    result_code: 201,
                    message: "Exercise added to workout".to_string(),
                    workout_exercise_id,
                }))
            }
            Some(_) => Err(Status::permission_denied(
                "Not authorized to modify this workout",
            )),
            None => Ok(tonic::Response::new(AddExerciseToWorkoutResponse {
                success: false,
                result_code: 404,
                message: "Workout not found".to_string(),
                workout_exercise_id: 0,
            })),
        }
    }

    async fn remove_exercise_from_workout(
        &self,
        request: Request<RemoveExerciseFromWorkoutRequest>,
    ) -> Result<Response<RemoveExerciseFromWorkoutResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership via workout_exercise -> workout
        let workout_exercise = WorkoutsRepository::get_workout_exercise::<_, AppError>(
            &mut *conn,
            req_data.workout_exercise_id,
        )
        .await?;

        match workout_exercise {
            Some(we) => {
                let workout =
                    WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, we.workout_id)
                        .await?;

                match workout {
                    Some(w) if w.created_by == user_id => {
                        WorkoutsRepository::delete_workout_exercise::<_, AppError>(
                            &mut *conn,
                            req_data.workout_exercise_id,
                            user_id,
                        )
                        .await?;

                        Ok(tonic::Response::new(RemoveExerciseFromWorkoutResponse {
                            success: true,
                            result_code: 200,
                            message: "Exercise removed from workout".to_string(),
                        }))
                    }
                    _ => Err(Status::permission_denied(
                        "Not authorized to modify this workout",
                    )),
                }
            }
            None => Ok(tonic::Response::new(RemoveExerciseFromWorkoutResponse {
                success: false,
                result_code: 404,
                message: "Workout exercise not found".to_string(),
            })),
        }
    }

    async fn update_workout_exercise(
        &self,
        request: Request<UpdateWorkoutExerciseRequest>,
    ) -> Result<Response<UpdateWorkoutExerciseResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership
        let workout_exercise = WorkoutsRepository::get_workout_exercise::<_, AppError>(
            &mut *conn,
            req_data.workout_exercise_id,
        )
        .await?;

        match workout_exercise {
            Some(we) => {
                let workout =
                    WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, we.workout_id)
                        .await?;

                match workout {
                    Some(w) if w.created_by == user_id => {
                        let notes = if req_data.notes.is_empty() {
                            None
                        } else {
                            Some(req_data.notes)
                        };

                        WorkoutsRepository::update_workout_exercise::<_, AppError>(
                            &mut *conn,
                            req_data.workout_exercise_id,
                            notes,
                            user_id,
                        )
                        .await?;

                        Ok(tonic::Response::new(UpdateWorkoutExerciseResponse {
                            success: true,
                            result_code: 200,
                            message: "Workout exercise updated".to_string(),
                        }))
                    }
                    _ => Err(Status::permission_denied(
                        "Not authorized to modify this workout",
                    )),
                }
            }
            None => Ok(tonic::Response::new(UpdateWorkoutExerciseResponse {
                success: false,
                result_code: 404,
                message: "Workout exercise not found".to_string(),
            })),
        }
    }

    async fn create_set(
        &self,
        request: Request<CreateSetRequest>,
    ) -> Result<Response<CreateSetResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership via workout_exercise -> workout
        let workout_exercise = WorkoutsRepository::get_workout_exercise::<_, AppError>(
            &mut *conn,
            req_data.workout_exercise_id,
        )
        .await?;

        match workout_exercise {
            Some(we) => {
                let workout =
                    WorkoutsRepository::get_workout::<_, AppError>(&mut *conn, we.workout_id)
                        .await?;

                match workout {
                    Some(w) if w.created_by == user_id => {
                        let position = WorkoutsRepository::get_next_set_position::<_, AppError>(
                            &mut *conn,
                            req_data.workout_exercise_id,
                        )
                        .await?;

                        let weight =
                            if req_data.weight.is_empty() {
                                None
                            } else {
                                Some(Decimal::from_str(&req_data.weight).map_err(|_| {
                                    Status::invalid_argument("Invalid weight format")
                                })?)
                            };

                        let reps = if req_data.reps == 0 {
                            None
                        } else {
                            Some(req_data.reps)
                        };

                        let notes = if req_data.notes.is_empty() {
                            None
                        } else {
                            Some(req_data.notes)
                        };

                        let set = ExerciseSet::new_weight_set(
                            req_data.workout_exercise_id,
                            position,
                            reps,
                            weight,
                            notes,
                            user_id,
                        );

                        let set_id =
                            WorkoutsRepository::create_set::<_, AppError>(&mut *conn, set).await?;

                        Ok(tonic::Response::new(CreateSetResponse {
                            success: true,
                            result_code: 201,
                            message: "Set created successfully".to_string(),
                            set_id,
                        }))
                    }
                    _ => Err(Status::permission_denied(
                        "Not authorized to modify this workout",
                    )),
                }
            }
            None => Ok(tonic::Response::new(CreateSetResponse {
                success: false,
                result_code: 404,
                message: "Workout exercise not found".to_string(),
                set_id: 0,
            })),
        }
    }

    async fn update_set(
        &self,
        request: Request<UpdateSetRequest>,
    ) -> Result<Response<UpdateSetResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership via set -> workout_exercise -> workout
        let set = WorkoutsRepository::get_set::<_, AppError>(&mut *conn, req_data.set_id).await?;

        match set {
            Some(s) => {
                let workout_exercise = WorkoutsRepository::get_workout_exercise::<_, AppError>(
                    &mut *conn,
                    s.workout_exercise_id,
                )
                .await?;

                match workout_exercise {
                    Some(we) => {
                        let workout = WorkoutsRepository::get_workout::<_, AppError>(
                            &mut *conn,
                            we.workout_id,
                        )
                        .await?;

                        match workout {
                            Some(w) if w.created_by == user_id => {
                                let weight = if req_data.weight.is_empty() {
                                    None
                                } else {
                                    Some(Decimal::from_str(&req_data.weight).map_err(|_| {
                                        Status::invalid_argument("Invalid weight format")
                                    })?)
                                };

                                let reps = if req_data.reps == 0 {
                                    None
                                } else {
                                    Some(req_data.reps)
                                };

                                let notes = if req_data.notes.is_empty() {
                                    None
                                } else {
                                    Some(req_data.notes)
                                };

                                WorkoutsRepository::update_set::<_, AppError>(
                                    &mut *conn,
                                    req_data.set_id,
                                    reps,
                                    weight,
                                    notes,
                                    user_id,
                                )
                                .await?;

                                Ok(tonic::Response::new(UpdateSetResponse {
                                    success: true,
                                    result_code: 200,
                                    message: "Set updated successfully".to_string(),
                                }))
                            }
                            _ => Err(Status::permission_denied(
                                "Not authorized to modify this workout",
                            )),
                        }
                    }
                    None => Ok(tonic::Response::new(UpdateSetResponse {
                        success: false,
                        result_code: 404,
                        message: "Workout exercise not found".to_string(),
                    })),
                }
            }
            None => Ok(tonic::Response::new(UpdateSetResponse {
                success: false,
                result_code: 404,
                message: "Set not found".to_string(),
            })),
        }
    }

    async fn delete_set(
        &self,
        request: Request<DeleteSetRequest>,
    ) -> Result<Response<DeleteSetResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        // Verify ownership via set -> workout_exercise -> workout
        let set = WorkoutsRepository::get_set::<_, AppError>(&mut *conn, req_data.set_id).await?;

        match set {
            Some(s) => {
                let workout_exercise = WorkoutsRepository::get_workout_exercise::<_, AppError>(
                    &mut *conn,
                    s.workout_exercise_id,
                )
                .await?;

                match workout_exercise {
                    Some(we) => {
                        let workout = WorkoutsRepository::get_workout::<_, AppError>(
                            &mut *conn,
                            we.workout_id,
                        )
                        .await?;

                        match workout {
                            Some(w) if w.created_by == user_id => {
                                WorkoutsRepository::delete_set::<_, AppError>(
                                    &mut *conn,
                                    req_data.set_id,
                                    user_id,
                                )
                                .await?;

                                Ok(tonic::Response::new(DeleteSetResponse {
                                    success: true,
                                    result_code: 200,
                                    message: "Set deleted successfully".to_string(),
                                }))
                            }
                            _ => Err(Status::permission_denied(
                                "Not authorized to modify this workout",
                            )),
                        }
                    }
                    None => Ok(tonic::Response::new(DeleteSetResponse {
                        success: false,
                        result_code: 404,
                        message: "Workout exercise not found".to_string(),
                    })),
                }
            }
            None => Ok(tonic::Response::new(DeleteSetResponse {
                success: false,
                result_code: 404,
                message: "Set not found".to_string(),
            })),
        }
    }
}

impl From<ExerciseSet> for ProtoExerciseSet {
    fn from(value: ExerciseSet) -> Self {
        Self {
            id: value.id,
            position: value.position,
            set_type: match value.set_type {
                SetType::Weight => "weight".to_string(),
                SetType::Timed => "timed".to_string(),
                SetType::Distance => "distance".to_string(),
            },
            reps: value.reps.unwrap_or(0),
            weight: value.weight.map(|w| w.to_string()).unwrap_or_default(),
            notes: value.notes.unwrap_or_default(),
        }
    }
}

impl From<WorkoutSummary> for ProtoWorkoutSummary {
    fn from(value: WorkoutSummary) -> Self {
        Self {
            id: value.id,
            workout_date: value.workout_date.format("%Y-%m-%d").to_string(),
            notes: value.notes.unwrap_or_default(),
            exercise_count: value.exercise_count,
            total_sets: value.total_sets,
        }
    }
}
