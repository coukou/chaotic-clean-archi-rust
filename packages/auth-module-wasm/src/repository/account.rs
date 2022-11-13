use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;
use auth_module::AccountRepository;
use web_sys::Storage;

pub struct LocalStorageAccountRepository {
    accounts: RwLock<HashMap<String, auth_module::Account>>,
}

fn get_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

fn restore_accounts() -> HashMap<String, auth_module::Account> {
    let storage = get_storage();
    match storage.get_item("accounts") {
        Ok(accounts_str) => {
            if let Some(account_str) = accounts_str {
                return serde_json::from_str(&account_str).unwrap();
            }
            HashMap::default()
        }
        Err(_) => HashMap::default(),
    }
}

impl LocalStorageAccountRepository {
    pub fn new() -> Self {
        LocalStorageAccountRepository {
            accounts: RwLock::new(restore_accounts()),
        }
    }

    fn persist(&self) -> Result<(), kernel::Error> {
        let storage = get_storage();
        let accounts = self.accounts.try_read().unwrap().clone();
        storage
            .set_item(
                "accounts",
                serde_json::to_string(&accounts).unwrap().as_str(),
            )
            .unwrap();
        Ok(())
    }
}

#[async_trait]
impl AccountRepository for LocalStorageAccountRepository {
    async fn create(&self, account: &auth_module::Account) -> Result<(), kernel::Error> {
        match self.accounts.try_write() {
            Ok(mut accounts) => {
                let key = &String::from(account.email.clone());
                accounts.insert(key.into(), account.clone());
            }
            Err(_) => Err(kernel::Error::RepositoryError)?,
        }
        self.persist()
    }

    async fn get(
        &self,
        account_email: &kernel::value::Email,
    ) -> Result<auth_module::Account, kernel::Error> {
        match self.accounts.try_read() {
            Ok(accounts) => {
                let key = &String::from(account_email.clone());
                if let Some(account) = accounts.get(key) {
                    return Ok(account.clone());
                }
                Err(kernel::Error::RepositoryError)?
            }
            Err(_) => Err(kernel::Error::RepositoryError)?,
        }
    }
}
