use kernel::value::Email;

use crate::{AccountRepository, Context};

pub async fn authenticate<C: Context>(
    context: &C,
    email: impl Into<String>,
    password: impl Into<String>,
) -> Result<String, kernel::Error> {
    let account = context
        .account_repository()
        .get(&Email::new(email)?)
        .await?;
    if !account.password.compare(password) {
        Err(kernel::Error::ApplicationError(
            "password doesnt match".into(),
        ))?
    }
    Ok("my_super_token".to_string())
}

// #[cfg(test)]
// mod test {

//     use std::sync::Arc;

//     use kernel::value::Email;
//     use lazy_static::lazy_static;

//     use crate::{account, value::Password, Account, Context, MockAccountRepository};

//     static VALID_EMAIL: &str = "toto@gmail.com";
//     static VALID_PASSWORD: &str = "JeSuisUnSuperMotDePasse123!";
//     static INVALID_PASSWORD: &str = "aaaa";

//     lazy_static! {
//         static ref VALID_ACCOUNT: Account = {
//             Account::new(account::Props {
//                 email: Email::new(VALID_EMAIL.to_string()).unwrap(),
//                 password: Password::new(VALID_PASSWORD.to_string(), 4).unwrap(),
//             })
//         };
//     }

//     #[tokio::test]
//     async fn should_fail_if_we_cant_retrieve_account() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock
//             .expect_get()
//             .returning(|_| Err(kernel::Error::RepositoryError));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         let error = super::authenticate(context, VALID_EMAIL, VALID_PASSWORD)
//             .await
//             .unwrap_err();
//         assert!(matches!(error, kernel::Error::RepositoryError))
//     }

//     #[tokio::test]
//     async fn should_fail_if_password_does_not_match() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock
//             .expect_get()
//             .returning(|_| Ok(VALID_ACCOUNT.clone()));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         let error = super::authenticate(
//             context,
//             VALID_EMAIL.to_string(),
//             INVALID_PASSWORD.to_string(),
//         )
//         .await
//         .unwrap_err();
//         assert!(matches!(error, kernel::Error::ApplicationError(_)))
//     }

//     #[tokio::test]
//     async fn should_authenticate() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock
//             .expect_get()
//             .returning(|_| Ok(VALID_ACCOUNT.clone()));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         super::authenticate(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
//             .await
//             .unwrap();
//     }
// }
