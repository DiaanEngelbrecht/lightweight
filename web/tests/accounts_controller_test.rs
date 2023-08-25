mod common;

use lightweight_web::protos::accounts::{CreateAccountRequest, CreateAccountResponse, LoginRequest, LoginResponse};
use lightweight_web::{
    controllers::accounts::AccountsController, protos::accounts::accounts_server::Accounts,
};
use tonic::Request;

#[tokio::test]
async fn account_registration() {
    // 1. Setup
    let server_ctx = common::setup("./config/test.yml").await;
    let controller = AccountsController::new();

    // 2. Build your request
    let request = Request::new(CreateAccountRequest {
        name: "Someone".to_string(),
        email: "someone@someplace.com".to_string(),
        password: "password".to_string(),
    });

    // 3. Hit your API
    let t_resp = controller.create_account(request).await;

    // 4. Make sure it works
    assert_eq!(true, t_resp.is_ok());
    let resp = t_resp.unwrap().into_inner();
    assert_eq!(
        resp,
        CreateAccountResponse {
            success: true,
            result_code: 201,
            message: "Account created successfully".to_string()
        }
    );

    let request = Request::new(LoginRequest {
        email: "someone@someplace.com".to_string(),
        password: "password".to_string(),
    });

    let l_resp = controller.login(request).await;
    assert_eq!(true, l_resp.is_ok());
    let login_resp = l_resp.unwrap().into_inner();

    assert_eq!(
        login_resp,
        LoginResponse {
            success: true,
            result_code: 200,
            message: "Login successfull!".to_string()
        }
    );

    let request = Request::new(LoginRequest {
        email: "someone@someplace.com".to_string(),
        password: "wrong_password".to_string(),
    });

    let l_resp = controller.login(request).await;
    assert_eq!(true, l_resp.is_ok());
    let login_resp = l_resp.unwrap().into_inner();

    assert_eq!(
        login_resp,
        LoginResponse {
            success: true,
            result_code: 403,
            message: "Login failed, invalid credentials.".to_string()
        }
    );
}
