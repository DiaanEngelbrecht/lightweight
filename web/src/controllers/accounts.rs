use crate::protos::accounts::accounts_server::Accounts;
use crate::protos::accounts::{CreateAccountRequest, CreateAccountResponse};
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
        todo!()
    }
}

