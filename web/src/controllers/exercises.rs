use super::AppError;
use crate::protos::exercises::exercises_server::Exercises;
use crate::protos::exercises::{
    Category, CreateExerciseRequest, CreateExerciseResponse, ListCategoriesRequest,
    ListCategoriesResponse, ListExercisesRequest, ListExercisesResponse,
};
use cali_core::store::get_conn;
use lightweight_store::repositories::exercises::contract::ExerciseRepositoryContract;
use lightweight_store::repositories::exercises::implementation::ExerciseRepository;
use lightweight_store::repositories::exercises::models::{Exercise, ExerciseCategory};
use tonic::async_trait;
use tonic::{Request, Response, Status};

cali_derive::controller!(ExercisesController);
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

        let new_exercise = Exercise::new(req_data.name.clone(), req_data.category_id);

        let mut conn = get_conn::<AppError>().await?;

        let _ =
            ExerciseRepository::create_exercise::<_, AppError>(&mut *conn, new_exercise).await?;

        return Ok(tonic::Response::new(CreateExerciseResponse {
            success: true,
            result_code: 201,
            message: "Exercise created successfully".to_string(),
        }));
    }

    async fn list_categories(
        &self,
        _request: Request<ListCategoriesRequest>,
    ) -> Result<Response<ListCategoriesResponse>, Status> {
        let mut conn = get_conn::<AppError>().await?;

        let categories = ExerciseRepository::list_categories::<_, AppError>(&mut *conn).await?;

        return Ok(tonic::Response::new(ListCategoriesResponse {
            categories: categories.into_iter().map(|v| v.into()).collect(),
        }));
    }
}

impl From<ExerciseCategory> for Category {
    fn from(value: ExerciseCategory) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
        }
    }
}
