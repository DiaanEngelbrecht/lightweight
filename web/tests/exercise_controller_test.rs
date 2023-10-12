mod common;

use lightweight_web::{
    controllers::{accounts::AccountsController, exercises::ExercisesController},
    protos::{
        accounts::{accounts_server::Accounts, LoginRequest},
        exercises::{exercises_server::Exercises, CreateExerciseRequest, CreateExerciseResponse},
    },
};
use tonic::Request;

#[tokio::test]
async fn exercise_creation() {
    // 1. Setup
    common::run("./config/test.yml", async {
        let accounts_controller = AccountsController::new();
        let exercises_controller = ExercisesController::new();

        let request = Request::new(LoginRequest {
            email: "someone@someplace.com".to_string(),
            password: "password".to_string(),
        });
        let l_resp = accounts_controller.login(request).await;
        let login_resp = l_resp.unwrap().into_inner();

        // 2. Build your request
        let request = Request::new(CreateExerciseRequest {
            name: "Someone".to_string(),
            category_id: 0,
            auth_token: login_resp.auth_token.clone(),
        });

        // 3. Hit your API
        let t_resp = exercises_controller.create_exercise(request).await;

        // 4. Make sure it works
        assert_eq!(true, t_resp.is_ok());
        let resp = t_resp.unwrap().into_inner();
        assert_eq!(
            resp,
            CreateExerciseResponse {
                success: true,
                result_code: 201,
                message: "Account created successfully".to_string()
            }
        );
    })
    .await;
}
