use salus::*;
use serde::{Deserialize, Serialize};

async fn index() -> String {
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

async fn receive_json(Json(HelloResponse { name }): Json<HelloResponse>) -> String {
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    let mut s = Salus::new();

    s.get("/", index);
    s.get("/json", json_hello);
    s.post("/hello", receive_json);

    s.serve("localhost", 8080).await
}
