use crate::protos::accounts::accounts_server::Accounts;
use crate::protos::accounts::{CreateAccountRequest, CreateAccountResponse};
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

        let new_account = Account {
            id: 0,
            name: req_data.name.clone(),
            email: req_data.email.clone(),
            password: req_data.password.clone(),
        };

        AccountsRepository::create_account(svr_ctx.db_pool, new_account).await;

        // request.into_inner().name

        todo!()
    }
}
