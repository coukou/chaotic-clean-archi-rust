use std::{collections::HashMap, sync::RwLock};

use crate::{Account, AccountRepository};

#[derive(Default)]
pub struct InMemoryAccountRepository {
    accounts: RwLock<HashMap<String, Account>>,
}

#[async_trait::async_trait]
impl AccountRepository for InMemoryAccountRepository {
    async fn create(&self, account: &crate::Account) -> Result<(), kernel::Error> {
        match self.accounts.try_write() {
            Ok(mut accounts) => {
                let key = &String::from(account.email.clone());
                if accounts.get(key).is_some() {
                    Err(kernel::Error::RepositoryError)?
                }
                accounts.insert(key.into(), account.clone());
                Ok(())
            }
            Err(_) => Err(kernel::Error::RepositoryError)?,
        }
    }

    async fn find(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<Option<crate::Account>, kernel::Error> {
        match self.accounts.try_read() {
            Ok(accounts) => {
                let key = &String::from(account_email.clone());
                if let Some(account) = accounts.get(key) {
                    return Ok(Some(account.clone()));
                }
            }
            Err(_) => Err(kernel::Error::RepositoryError)?,
        }
        Ok(None)
    }

    async fn get(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<crate::Account, kernel::Error> {
        let account = self.find(account_email).await?;
        if let Some(account) = account {
            return Ok(account);
        }
        Err(kernel::Error::RepositoryError)?
    }
}
