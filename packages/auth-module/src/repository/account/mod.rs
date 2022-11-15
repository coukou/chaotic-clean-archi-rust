use std::sync::Arc;

use enum_dispatch::enum_dispatch;

pub mod implementation;

#[async_trait::async_trait]
#[enum_dispatch]
#[mockall::automock]
pub trait AccountRepository {
    async fn create(&self, account: &crate::Account) -> Result<(), kernel::Error>;
    async fn get(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<crate::Account, kernel::Error>;
    async fn find(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<Option<crate::Account>, kernel::Error>;
    // async fn delete(&self, account: &crate::Account) -> Result<(), kernel::Error>;
}

#[enum_dispatch(AccountRepository)]
pub enum AccountRepositoryContainer {
    MockAccountRepository,
}

pub type DynAccountRepository = Arc<dyn AccountRepository + Send + Sync>;
