use std::{fmt::format, net::SocketAddr};

use salus::*;
use serde::{Deserialize, Serialize};

async fn handle() -> String {
    "Hello, world!".to_string()
}

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    name: String,
}

async fn json_hello() -> Json<HelloResponse> {
    HelloResponse {
        name: "World".to_string(),
    }
    .into()
}

async fn receive_json(Json(input): Json<HelloResponse>) -> String {
    format!("Hello {}", input.name)
}

#[tokio::main]
async fn main() {
    use http::Method;
    let mut s = Salus::new();

    s.add_route("/", Method::GET, handle);
    s.add_route("/json", Method::GET, json_hello);
    s.add_route("/hello", Method::POST, receive_json);

    for route in s.routes() {
        println!("{}: {}", route.method, route.path);
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    s.serve(addr).await;
}
