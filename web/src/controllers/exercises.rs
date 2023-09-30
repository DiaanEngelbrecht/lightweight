use crate::config::Config;
use crate::protos::exercises::exercises_server::Exercises;
use crate::protos::exercises::{
    CreateExerciseRequest, CreateExerciseResponse, ListExercisesRequest, ListExercisesResponse,
};
use chrono::Utc;
use flair_core::store::get_conn;
use lightweight_store::repositories::exercises::contract::ExerciseRepositoryContract;
use lightweight_store::repositories::exercises::implementation::ExerciseRepository;
use lightweight_store::repositories::exercises::models::Exercise;
use tonic::async_trait;
use tonic::{Request, Response, Status};

use super::AppError;

flair_derive::controller!(ExercisesController);
#[async_trait]
impl Exercises for ExercisesController {
    async fn list_exercises(
        &self,
        _request: Request<ListExercisesRequest>,
    ) -> Result<Response<ListExercisesResponse>, Status> {
        todo!()
    }

    async fn create_exercise(
        &self,
        request: Request<CreateExerciseRequest>,
    ) -> Result<Response<CreateExerciseResponse>, Status> {
        let req_data = request.into_inner();

        let new_exercise = Exercise {
            id: 0,
            name: req_data.name.clone(),
            category_id: req_data.category_id,
            updated_at: Utc::now().naive_utc(),
            updated_by: 1,
            deleted_at: None,
            deleted_by: None,
        };

        let mut conn = get_conn::<AppError>().await?;

        let _ = ExerciseRepository::create_exercise::<_, AppError>(&mut *conn, new_exercise).await?;

        return Ok(tonic::Response::new(CreateExerciseResponse {
            success: true,
            result_code: 201,
            message: "Exercise created successfully".to_string(),
        }));
    }
}
