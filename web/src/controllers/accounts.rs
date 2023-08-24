use crate::protos::accounts::{
    accounts_server::Accounts, CreateAccountRequest, CreateAccountResponse, LoginRequest,
    LoginResponse,
};
use chrono::{DateTime, Utc, Duration};
use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey};
use lightweight_store::repositories::accounts::contract::AccountsRepositoryContract;
use lightweight_store::repositories::accounts::implementation::AccountsRepository;
use lightweight_store::repositories::accounts::models::Account;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use sha2::Sha256;
use tonic::{async_trait, Request, Response, Status};

flair_derive::controller!(AccountsController);
#[async_trait]
impl Accounts for AccountsController {
    #[endpoint]
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
        let svr_ctx = flair_core::SERVER_CONTEXT.with(|ctx| ctx.clone());

        // Prepare salt
        let mut salt: [u8; 64] = [0; 64];
        let mut rng = StdRng::from_entropy();
        rng.fill_bytes(&mut salt);

        let req_data = request.into_inner();
        let password = req_data.password.as_bytes();

        let config = argon2::Config::default();
        let hashed_password = match argon2::hash_encoded(password, &salt, &config) {
            Ok(h) => h,
            Err(e) => {
                log::error!("Could not generate password hash {}", e);
                return Ok(tonic::Response::new(CreateAccountResponse {
                    success: false,
                    message: format!("Could not generate password hash {}", e),
                    result_code: 500,
                }));
            }
        };

        let new_account = Account {
            id: 0,
            name: req_data.name.clone(),
            salt: salt.to_vec(),
            email: req_data.email.clone(),
            password_hash: hashed_password.to_string(),
        };

        if let Ok(mut conn) = svr_ctx.db_pool.acquire().await {
            let create_result = AccountsRepository::create_account(&mut *conn, new_account).await;
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
                }
            }
        }

        return Ok(tonic::Response::new(CreateAccountResponse {
            success: false,
            message: "Could not create account".to_string(),
            result_code: 500,
        }));
    }

    #[endpoint]
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req_data = request.into_inner();

        let svr_ctx = flair_core::SERVER_CONTEXT.with(|ctx| ctx.clone());

        let login_result: Result<&str, &str> = if let Ok(mut conn) = svr_ctx.db_pool.acquire().await
        {
            if let Ok(Some(account)) =
                AccountsRepository::get_account(&mut *conn, req_data.email).await
            {
                if Ok(true)
                    == argon2::verify_encoded(&account.password_hash, req_data.password.as_bytes())
                {
                    Ok("Login successfull!")
                } else {
                    Err("Login failed, invalid credentials.")
                }
            } else {
                Err("Login failed, invalid credentials.")
            }
        } else {
            Err("Login failed due to internal server error, please try again later")
        };

        match login_result {
            Ok(message) => {
                // Generate the jwt
                //
                //
                let utc: DateTime<Utc> = Utc::now() + Duration::minutes(5);
                let key: Hmac<Sha256> = Hmac::new_from_slice(svr_ctx).unwrap();
                let mut claims = Claims::new(RegisteredClaims {
                    issuer: None,
                    subject: None,
                    audience: None,
                    expiration: Some(utc.timestamp() as u64),
                    not_before: Some(utc.timestamp() as u64),
                    issued_at: Some(utc.timestamp() as u64),
                    json_web_token_id: None,
                });
                let token_str = claims.sign_with_key(&key);

                return Ok(tonic::Response::new(LoginResponse {
                    success: true,
                    result_code: 200,
                    message: message.to_string(),
                }));
            }
            Err(message) => {
                return Ok(tonic::Response::new(LoginResponse {
                    success: true,
                    message: message.to_string(),
                    result_code: 403,
                }));
            }
        }
    }
}
