use crate::handlers::user_handlers::{
    create_user, delete_user, get_user, list_users, patch_user, update_user,
};
use axum::{Router, routing::get};

pub fn user_routes() -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route(
            "/{id}",
            get(get_user)
                .put(update_user)
                .patch(patch_user)
                .delete(delete_user),
        )
}
