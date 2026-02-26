use crate::handlers::transport_company_handlers::{
    create_driver, create_transport_company, create_vehicle, delete_driver,
    delete_transport_company, delete_vehicle, get_driver, get_transport_company, get_vehicle,
    list_drivers, list_transport_companies, list_vehicles, patch_driver, patch_transport_company,
    patch_vehicle, update_driver, update_transport_company, update_vehicle,
};
use axum::{Router, routing::get};

pub fn transport_company_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(list_transport_companies).post(create_transport_company),
        )
        .route(
            "/{id}",
            get(get_transport_company)
                .put(update_transport_company)
                .patch(patch_transport_company)
                .delete(delete_transport_company),
        )
        .route(
            "/{transport_company_id}/vehicles",
            get(list_vehicles).post(create_vehicle),
        )
        .route(
            "/vehicles/{id}",
            get(get_vehicle)
                .put(update_vehicle)
                .patch(patch_vehicle)
                .delete(delete_vehicle),
        )
        .route(
            "/{transport_company_id}/drivers",
            get(list_drivers).post(create_driver),
        )
        .route(
            "/drivers/{id}",
            get(get_driver)
                .put(update_driver)
                .patch(patch_driver)
                .delete(delete_driver),
        )
}
