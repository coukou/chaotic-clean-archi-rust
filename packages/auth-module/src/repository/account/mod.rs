use std::sync::Arc;

pub mod implementation;

#[mockall::automock]
#[async_trait::async_trait]
pub trait AccountRepository {
    async fn create(&self, account: &crate::Account) -> Result<(), kernel::Error>;
    async fn get(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<crate::Account, kernel::Error>;
    // async fn delete(&self, account: &crate::Account) -> Result<(), kernel::Error>;
}

pub type DynAccountRepository = Arc<dyn AccountRepository + Send + Sync>;
