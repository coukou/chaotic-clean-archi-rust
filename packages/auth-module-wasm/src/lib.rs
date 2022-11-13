mod repository;

use std::sync::Arc;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

lazy_static! {
    static ref AUTH_CONTEXT: Arc<auth_module::Context> = Arc::new(auth_module::Context {
        account_repository: Arc::new(repository::account::LocalStorageAccountRepository::new())
    });
}

pub struct Error(kernel::Error);

impl From<Error> for JsError {
    fn from(error: Error) -> Self {
        match error.0 {
            kernel::Error::ParseError => JsError::new("parse_error"),
            kernel::Error::RepositoryError => JsError::new("repository_error"),
            kernel::Error::ApplicationError(reason) => {
                JsError::new(format!("application_error: {}", reason).as_str())
            }
        }
    }
}

#[wasm_bindgen]
pub async fn register(email: &str, password: &str) -> Result<(), JsError> {
    auth_module::usecase::register(
        AUTH_CONTEXT.clone(),
        email.to_string(),
        password.to_string(),
    )
    .await
    .map_err(Error)?;

    Ok(())
}

#[wasm_bindgen]
pub async fn authenticate(email: &str, password: &str) -> Result<String, JsError> {
    let token = auth_module::usecase::authenticate(
        AUTH_CONTEXT.clone(),
        email.to_string(),
        password.to_string(),
    )
    .await
    .map_err(Error)?;

    Ok(token)
}
