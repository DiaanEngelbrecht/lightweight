use crate::protos::exercises::exercises_server::Exercises;
use crate::protos::exercises::{
    CreateExerciseRequest, CreateExerciseResponse, ListExercisesRequest, ListExercisesResponse,
};
use tonic::async_trait;
use tonic::{Request, Response, Status};

flair_derive::controller!(ExercisesController);
#[async_trait]
impl Exercises for ExercisesController {
    async fn list_exercises(
        &self,
        request: Request<ListExercisesRequest>,
    ) -> Result<Response<ListExercisesResponse>, Status> {
        todo!()
    }

    async fn create_exercise(
        &self,
        request: Request<CreateExerciseRequest>,
    ) -> Result<Response<CreateExerciseResponse>, Status> {
        todo!()
    }
}

