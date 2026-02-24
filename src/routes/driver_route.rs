use crate::handlers::driver_handlers::{
    create_driver, delete_driver, get_driver, list_drivers, patch_driver, update_driver,
};
use axum::{Router, routing::get};

pub fn driver_routes() -> Router {
    Router::new()
        .route("/", get(list_drivers).post(create_driver))
        .route(
            "/{id}",
            get(get_driver)
                .put(update_driver)
                .patch(patch_driver)
                .delete(delete_driver),
        )
}
