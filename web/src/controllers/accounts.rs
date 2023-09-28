use crate::config::Config;
use crate::protos::accounts::{
    accounts_server::Accounts, CreateAccountRequest, CreateAccountResponse, LoginRequest,
    LoginResponse,
};
use chrono::{DateTime, Duration, Utc};
use flair_core::config::get_config;
use flair_core::store::get_conn;
use hmac::{Hmac, Mac};
use jwt::{Claims, RegisteredClaims, SignWithKey};
use lightweight_store::repositories::accounts::contract::AccountsRepositoryContract;
use lightweight_store::repositories::accounts::implementation::AccountsRepository;
use lightweight_store::repositories::accounts::models::Account;
use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};
use sha2::Sha256;
use tonic::{async_trait, Request, Response, Status};

use super::AppError;

flair_derive::controller!(AccountsController);

#[async_trait]
impl Accounts for AccountsController {
    async fn create_account(
        &self,
        request: Request<CreateAccountRequest>,
    ) -> Result<Response<CreateAccountResponse>, Status> {
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
            email: req_data.email.clone(),
            password_hash: hashed_password.to_string(),
        };

        let mut conn = get_conn::<AppError, Config>().await?;
        let _ = AccountsRepository::create_account::<_, AppError>(&mut *conn, new_account).await?;
        return Ok(tonic::Response::new(CreateAccountResponse {
            success: true,
            result_code: 201,
            message: "Account created successfully".to_string(),
        }));
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req_data = request.into_inner();

        let mut conn = get_conn::<AppError, Config>().await?;
        let login_result: Result<&str, &str> = if let Some(account) =
            AccountsRepository::get_account::<_, AppError>(&mut *conn, req_data.email).await?
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
        };

        match login_result {
            Ok(message) => {
                // Generate the jwt
                let utc: DateTime<Utc> = Utc::now() + Duration::minutes(5);
                let key = get_config::<Config>().jwt_secret.clone();
                let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes()).unwrap();

                let claims = Claims::new(RegisteredClaims {
                    issuer: None,
                    subject: None,
                    audience: None,
                    expiration: Some(utc.timestamp() as u64),
                    not_before: Some(utc.timestamp() as u64),
                    issued_at: Some(utc.timestamp() as u64),
                    json_web_token_id: None,
                });
                let token_str = claims
                    .sign_with_key(&key)
                    .expect("Should have been able to sign");

                return Ok(tonic::Response::new(LoginResponse {
                    success: true,
                    auth_token: token_str.to_string(),
                    result_code: 200,
                    message: message.to_string(),
                }));
            }
            Err(message) => {
                return Ok(tonic::Response::new(LoginResponse {
                    success: true,
                    auth_token: "".to_string(),
                    message: message.to_string(),
                    result_code: 403,
                }));
            }
        }
    }
}
