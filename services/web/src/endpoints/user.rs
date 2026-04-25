// mod greet_service {
//     tonic::include_proto!("projectx");
// }

// use axum::{Json, Router, routing::get};
// use greet_service::{HelloRequest, greeter_client::GreeterClient};
// use serde::Serialize;

// use crate::{config, error::AppError};

// #[derive(Serialize)]
// struct HelloResponse {
//     message: String,
// }

// pub fn routes() -> Router {
//     Router::new().nest("/users", Router::new().route("/", get(get_user)))
// }

// async fn get_user() -> Result<Json<HelloResponse>, AppError> {
//     let cfg = config::get_or_init();

//     let client = GreeterClient::connect(cfg.grpc_url.clone())
//         .await
//         .map_err(|e| AppError::GrpcConnection(format!("Failed to connect to gRPC: {}", e)))?;

//     let request = tonic::Request::new(HelloRequest {
//         name: "Monir Hossain".into(),
//     });

//     let response = client
//         .clone()
//         .say_hello(request)
//         .await
//         .map_err(|e| AppError::GrpcCall(format!("gRPC call failed: {}", e)))?;

//     let message = response.into_inner().message;
//     Ok(Json(HelloResponse { message }))
// }
