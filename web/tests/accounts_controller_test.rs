mod common;

use lightweight_web::protos::accounts::{CreateAccountRequest, CreateAccountResponse};
use lightweight_web::{
    controllers::accounts::AccountsController, protos::accounts::accounts_server::Accounts,
};
use tonic::Request;

#[tokio::test]
async fn account_registration() {
    let server_ctx = common::setup("./config/test.yml").await;
    let controller = AccountsController::new(server_ctx);

    let request = Request::new(CreateAccountRequest {
        name: "Someone".to_string(),
        email: "someone@someplace.com".to_string(),
        password: "password".to_string(),
    });
    let t_resp = controller.create_account(request).await;

    // Hit the API for the test
    assert_eq!(true, t_resp.is_ok());

    let resp = t_resp.unwrap().into_inner();
    assert_eq!(
        resp,
        CreateAccountResponse {
            success: true,
            result_code: 200,
            message: "Some message".to_string()
        }
    );
}
