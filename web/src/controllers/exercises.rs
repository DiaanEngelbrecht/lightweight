use super::AppError;
use crate::auth::{authenticate_request, try_authenticate_request};
use crate::config::Config;
use crate::protos::exercises::exercises_server::Exercises;
use crate::protos::exercises::{
    Category, CreateCategoryRequest, CreateCategoryResponse, CreateExerciseRequest,
    CreateExerciseResponse, DeleteCategoryRequest, DeleteCategoryResponse, DeleteExerciseRequest,
    DeleteExerciseResponse, Exercise as ProtoExercise, ListCategoriesRequest,
    ListCategoriesResponse, ListExercisesRequest, ListExercisesResponse, UpdateCategoryRequest,
    UpdateCategoryResponse, UpdateExerciseRequest, UpdateExerciseResponse,
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
    // Category endpoints
    async fn list_categories(
        &self,
        request: Request<ListCategoriesRequest>,
    ) -> Result<Response<ListCategoriesResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = try_authenticate_request(&request, &config.jwt_secret)?;

        let mut conn = get_conn::<AppError>().await?;
        let categories =
            ExerciseRepository::list_categories::<_, AppError>(&mut *conn, user_id).await?;

        return Ok(tonic::Response::new(ListCategoriesResponse {
            categories: categories.into_iter().map(|v| v.into()).collect(),
        }));
    }

    async fn create_category(
        &self,
        request: Request<CreateCategoryRequest>,
    ) -> Result<Response<CreateCategoryResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let new_category = ExerciseCategory::new(req_data.name.clone(), Some(user_id));

        let mut conn = get_conn::<AppError>().await?;
        let category_id =
            ExerciseRepository::create_category::<_, AppError>(&mut *conn, new_category).await?;

        return Ok(tonic::Response::new(CreateCategoryResponse {
            success: true,
            result_code: 201,
            message: "Category created successfully".to_string(),
            category_id,
        }));
    }

    async fn update_category(
        &self,
        request: Request<UpdateCategoryRequest>,
    ) -> Result<Response<UpdateCategoryResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        ExerciseRepository::update_category::<_, AppError>(
            &mut *conn,
            req_data.category_id,
            req_data.name,
            user_id,
        )
        .await?;

        return Ok(tonic::Response::new(UpdateCategoryResponse {
            success: true,
            result_code: 200,
            message: "Category updated successfully".to_string(),
        }));
    }

    async fn delete_category(
        &self,
        request: Request<DeleteCategoryRequest>,
    ) -> Result<Response<DeleteCategoryResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        ExerciseRepository::delete_category::<_, AppError>(
            &mut *conn,
            req_data.category_id,
            user_id,
        )
        .await?;

        return Ok(tonic::Response::new(DeleteCategoryResponse {
            success: true,
            result_code: 200,
            message: "Category deleted successfully".to_string(),
        }));
    }

    // Exercise endpoints
    async fn list_exercises(
        &self,
        request: Request<ListExercisesRequest>,
    ) -> Result<Response<ListExercisesResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = try_authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        let exercises = ExerciseRepository::list_exercises::<_, AppError>(
            &mut *conn,
            req_data.category_id,
            user_id,
        )
        .await?;

        return Ok(tonic::Response::new(ListExercisesResponse {
            exercises: exercises.into_iter().map(|v| v.into()).collect(),
        }));
    }

    async fn create_exercise(
        &self,
        request: Request<CreateExerciseRequest>,
    ) -> Result<Response<CreateExerciseResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let new_exercise =
            Exercise::new(req_data.name.clone(), req_data.category_id, Some(user_id));

        let mut conn = get_conn::<AppError>().await?;
        let exercise_id =
            ExerciseRepository::create_exercise::<_, AppError>(&mut *conn, new_exercise).await?;

        return Ok(tonic::Response::new(CreateExerciseResponse {
            success: true,
            result_code: 201,
            message: "Exercise created successfully".to_string(),
            exercise_id,
        }));
    }

    async fn update_exercise(
        &self,
        request: Request<UpdateExerciseRequest>,
    ) -> Result<Response<UpdateExerciseResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        ExerciseRepository::update_exercise::<_, AppError>(
            &mut *conn,
            req_data.exercise_id,
            req_data.name,
            user_id,
        )
        .await?;

        return Ok(tonic::Response::new(UpdateExerciseResponse {
            success: true,
            result_code: 200,
            message: "Exercise updated successfully".to_string(),
        }));
    }

    async fn delete_exercise(
        &self,
        request: Request<DeleteExerciseRequest>,
    ) -> Result<Response<DeleteExerciseResponse>, Status> {
        let config = cali_core::helpers::get_context(|config: &Config| config.clone());
        let user_id = authenticate_request(&request, &config.jwt_secret)?;

        let req_data = request.into_inner();
        let mut conn = get_conn::<AppError>().await?;

        ExerciseRepository::delete_exercise::<_, AppError>(
            &mut *conn,
            req_data.exercise_id,
            user_id,
        )
        .await?;

        return Ok(tonic::Response::new(DeleteExerciseResponse {
            success: true,
            result_code: 200,
            message: "Exercise deleted successfully".to_string(),
        }));
    }
}

impl From<ExerciseCategory> for Category {
    fn from(value: ExerciseCategory) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            created_by: value.created_by.unwrap_or(0),
        }
    }
}

impl From<Exercise> for ProtoExercise {
    fn from(value: Exercise) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            category_id: value.category_id,
            created_by: value.created_by.unwrap_or(0),
        }
    }
}
