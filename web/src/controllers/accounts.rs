use crate::protos::accounts::accounts_server::Accounts;
use crate::protos::accounts::{CreateAccountRequest, CreateAccountResponse};
use lightweight_store::repositories::accounts::contract::AccountsRepositoryContract;
use lightweight_store::repositories::accounts::implementation::AccountsRepository;
use lightweight_store::repositories::accounts::models::Account;
use tonic::async_trait;
use tonic::{Request, Response, Status};

flair_derive::controller!(AccountsController);
#[async_trait]
impl Accounts for AccountsController {
    #[endpoint]
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        log::info!("Inside account setup");
        let svr_ctx = flair_core::SERVER_CONTEXT.with(|ctx| ctx.clone());

        let req_data = request.into_inner();

        let password = req_data.password.as_bytes();
        let salt = req_data.name.as_bytes();
        let config = argon2::Config::default();

        // let matches = argon2::verify_encoded(&hash, password).unwrap();
        let hashed_password = match argon2::hash_encoded(password, salt, &config) {
            Ok(h) => h,
            Err(e) => {
                log::error!("Could not generate password hash {}", e);
                return Ok(tonic::Response::new(CreateAccountResponse {
                    success: false,
                    message: "Could not generate password hash".to_string(),
                    result_code: 500,
                }));
            }
        };

        let new_account = Account {
            id: 0,
            name: req_data.name.clone(),
            email: req_data.email.clone(),
            password: hashed_password.to_string(),
        };

        if let Ok(mut conn) = svr_ctx.db_pool.acquire().await {
            let create_result = AccountsRepository::create_account(&mut conn, new_account).await;
            match create_result {
                Ok(_) => {
                    return Ok(tonic::Response::new(CreateAccountResponse {
                        success: true,
                        result_code: 201,
                        message: "Account created successfully".to_string(),
                    }))
                }
                Err(e) => {
                    log::error!("Could not insert new account into DB, err => {}", e)
                },
            }

        }

        return Ok(tonic::Response::new(CreateAccountResponse {
            success: false,
            message: "Could not create account".to_string(),
            result_code: 500,
        }));
    }
}
