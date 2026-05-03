use axum::Router;

mod api;
mod data;
mod util;

use crate::api::house::controller::house_controller;
use crate::api::player::controller::player_controller;
use crate::api::position::controller::position_controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api/v1/quidditchplayers/house", house_controller::router())
        .nest(
            "/api/v1/quidditchplayers/player",
            player_controller::router(),
        )
        .nest(
            "/api/v1/quidditchplayers/position",
            position_controller::router(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    server_details();
    axum::serve(listener, app).await.unwrap();
}

fn server_details() {
    println!("Server is up!")
}
