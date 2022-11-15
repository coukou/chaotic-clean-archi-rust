use kernel::value::Email;

use crate::{account, value::Password, Account, AccountRepository, Context};
pub async fn register<C: Context>(
    context: &C,
    email: impl Into<String>,
    password: impl Into<String>,
) -> Result<(), kernel::Error> {
    let email = Email::new(email)?;

    if context.account_repository().find(&email).await?.is_some() {
        Err(kernel::Error::ApplicationError(
            "email already in use".to_string(),
        ))?
    }

    let account = Account::new(account::Props {
        email: email,
        password: Password::new(password, 4)?,
    });
    context.account_repository().create(&account).await
}

// #[cfg(test)]
// mod test {

//     use std::sync::Arc;

//     use kernel::value::Email;
//     use lazy_static::lazy_static;

//     use crate::{account, value::Password, Account, Context, MockAccountRepository};

//     static VALID_EMAIL: &str = "toto@gmail.com";
//     static INVALID_EMAIL: &str = "toto";
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
//     async fn should_fail_on_invalid_email() {
//         let account_repo_mock = MockAccountRepository::default();
//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         assert_eq!(
//             super::register(
//                 context,
//                 INVALID_EMAIL.to_string(),
//                 VALID_PASSWORD.to_string(),
//             )
//             .await
//             .unwrap_err(),
//             kernel::Error::ParseError
//         );
//     }

//     #[tokio::test]
//     async fn should_fail_if_email_already_taken() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock
//             .expect_find()
//             .returning(|_| Ok(Some(VALID_ACCOUNT.clone())));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         assert!(matches!(
//             super::register(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
//                 .await
//                 .unwrap_err(),
//             kernel::Error::ApplicationError(_)
//         ))
//     }

//     #[tokio::test]
//     async fn should_fail_on_invalid_password() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock.expect_find().returning(|_| Ok(None));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         assert_eq!(
//             super::register(
//                 context,
//                 VALID_EMAIL.to_string(),
//                 INVALID_PASSWORD.to_string()
//             )
//             .await
//             .unwrap_err(),
//             kernel::Error::ParseError
//         )
//     }

//     #[tokio::test]
//     async fn should_fail_on_repo_create_fail() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock.expect_find().returning(|_| Ok(None));
//         account_repo_mock
//             .expect_create()
//             .returning(|_| Err(kernel::Error::RepositoryError));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         assert_eq!(
//             super::register(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
//                 .await
//                 .unwrap_err(),
//             kernel::Error::RepositoryError
//         )
//     }

//     #[tokio::test]
//     async fn should_register_account() {
//         let mut account_repo_mock = MockAccountRepository::default();
//         account_repo_mock.expect_find().returning(|_| Ok(None));
//         account_repo_mock.expect_create().returning(|_| Ok(()));

//         let context = Arc::new(Context {
//             account_repository: Arc::new(account_repo_mock),
//         });

//         super::register(context, VALID_EMAIL.to_string(), VALID_PASSWORD.to_string())
//             .await
//             .unwrap();
//     }
// }
