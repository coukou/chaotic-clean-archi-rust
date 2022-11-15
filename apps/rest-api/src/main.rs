use std::{net::SocketAddr, sync::Arc};

use auth_module::InMemoryAccountRepository;
use axum::{routing::post, Extension, Json, Router};
use lazy_static::lazy_static;
use serde::Deserialize;

struct AuthContext {
    account_repository: InMemoryAccountRepository,
}

impl auth_module::Context for AuthContext {
    type AccountRepository = InMemoryAccountRepository;

    fn account_repository(&self) -> &Self::AccountRepository {
        &self.account_repository
    }
}

struct SharedContext {
    pub auth_context: AuthContext,
}

lazy_static! {
    static ref SHARED_CONTEXT: Arc<SharedContext> = Arc::new(SharedContext {
        auth_context: AuthContext {
            account_repository: InMemoryAccountRepository::default()
        },
    });
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/register", post(register))
        .route("/auth", post(authenticate))
        .layer(Extension(SHARED_CONTEXT.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("rest-api server listenin on: {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

async fn register(
    Extension(context): Extension<Arc<SharedContext>>,
    Json(payload): Json<RegisterRequest>,
) {
    auth_module::usecase::register(
        &context.clone().auth_context,
        payload.email,
        payload.password,
    )
    .await
    .unwrap();
}

#[derive(Deserialize)]
struct AuthRequest {
    email: String,
    password: String,
}

async fn authenticate(
    Extension(context): Extension<Arc<SharedContext>>,
    Json(payload): Json<AuthRequest>,
) -> String {
    auth_module::usecase::authenticate(
        &context.clone().auth_context,
        payload.email,
        payload.password,
    )
    .await
    .unwrap()
}
