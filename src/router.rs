use axum::{routing::{get, post},Router};

use crate::{
    handlers::{auth::login, users::*},
    middleware::auth::auth_middleware,
    state::AppState
};

pub fn create_router(state: AppState) -> Router {

    let public = Router::new()
        .route("/signup", post(create_user))
        .route("/login", post(login));

    let protected = Router::new()
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user).delete(delete_user))
        .route("/greet", post(greet_user))
        .layer(axum::middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public)
        .merge(protected)
        .with_state(state)

}