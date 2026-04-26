use axum::Router;

mod api;
use api::house::controller::house_controller::router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api/v1/quidditchplayers/house", router());
             
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    server_details();
    axum::serve(listener, app).await.unwrap();
}

fn server_details() {
    println!("Server is up!")
}
