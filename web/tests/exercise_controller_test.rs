mod common;

use lightweight_web::{
    controllers::{accounts::AccountsController, exercises::ExercisesController},
    protos::{
        accounts::{accounts_server::Accounts, LoginRequest},
        exercises::{
            exercises_server::Exercises, CreateExerciseRequest, CreateExerciseResponse,
            ListCategoriesRequest,
        },
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
        let _login_resp = l_resp.unwrap().into_inner();

        let request = Request::new(ListCategoriesRequest {
            filter: "".to_string(),
        });
        let c_resp = exercises_controller.list_categories(request).await;
        assert_eq!(true, c_resp.is_ok());
        let c_resp_inner = c_resp.unwrap().into_inner();
        println!("{:?}", c_resp_inner);

        let request = Request::new(CreateExerciseRequest {
            name: "Someone".to_string(),
            category_id: c_resp_inner.categories.first().unwrap().id,
        });
        let t_resp = exercises_controller.create_exercise(request).await;
        assert_eq!(true, t_resp.is_ok());
        let resp = t_resp.unwrap().into_inner();
        assert_eq!(
            resp,
            CreateExerciseResponse {
                success: true,
                result_code: 201,
                message: "Exercise created successfully".to_string()
            }
        );
    })
    .await;
}
