use std::sync::Arc;

use kernel::value::Email;

use crate::{account, value::Password, Account, Context};

pub async fn register(
    context: Arc<Context>,
    email: impl Into<String>,
    password: impl Into<String>,
) -> Result<(), kernel::Error> {
    let account = Account::new(account::Props {
        email: Email::new(email)?,
        password: Password::new(password, 4)?,
    });
    context.account_repository.create(&account).await
}

#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::{Context, MockAccountRepository};

    static VALID_EMAIL: &str = "toto@gmail.com";
    static INVALID_EMAIL: &str = "toto";
    static VALID_PASSWORD: &str = "JeSuisUnSuperMotDePasse123!";
    static INVALID_PASSWORD: &str = "aaaa";

    #[tokio::test]
    async fn should_fail_on_invalid_email() {
        let account_repo_mock = MockAccountRepository::default();
        let context = Arc::new(Context {
            account_repository: Arc::new(account_repo_mock),
        });

        assert_eq!(
            super::register(
                context,
                INVALID_EMAIL.to_string(),
                VALID_PASSWORD.to_string(),
            )
            .await
            .unwrap_err(),
            kernel::Error::ParseError
        );
    }

    #[tokio::test]
    async fn should_fail_on_invalid_password() {
        let account_repo_mock = MockAccountRepository::default();
        let context = Arc::new(Context {
            account_repository: Arc::new(account_repo_mock),
        });

        assert_eq!(
            super::register(
                context,
                VALID_EMAIL.to_string(),
                INVALID_PASSWORD.to_string()
            )
            .await
            .unwrap_err(),
            kernel::Error::ParseError
        )
    }

    #[tokio::test]
    async fn should_fail_on_repo_create_fail() {
        let mut account_repo_mock = MockAccountRepository::default();
        account_repo_mock
            .expect_create()
            .returning(|_| Err(kernel::Error::RepositoryError));

        let context = Arc::new(Context {
            account_repository: Arc::new(account_repo_mock),
        });

        assert_eq!(
            super::register(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
                .await
                .unwrap_err(),
            kernel::Error::RepositoryError
        )
    }

    #[tokio::test]
    async fn should_register_account() {
        let mut account_repo_mock = MockAccountRepository::default();
        account_repo_mock.expect_create().returning(|_| Ok(()));

        let context = Arc::new(Context {
            account_repository: Arc::new(account_repo_mock),
        });

        super::register(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
            .await
            .unwrap();
    }
}
