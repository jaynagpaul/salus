use std::{
    collections::HashMap,
    sync::atomic::{AtomicI64, Ordering},
};

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

async fn check_database(Json(HelloResponse { name }): Json<HelloResponse>) -> Option<String> {
    let mut db = HashMap::new();

    db.insert("Jay", 1);
    db.insert("John", 2);
    db.insert("Jac", 3);
    db.insert("Jeff", 4);

    db.get(&name as &str).map(|i| format!("{} is {}", name, i))
}

async fn increment(state: State<Counter>) -> String {
    state.count.fetch_add(1, Ordering::SeqCst);

    state.count.load(Ordering::SeqCst).to_string()
}

struct Counter {
    count: AtomicI64,
}

#[tokio::main]
async fn main() {
    let mut s = Salus::new();

    s.manage(Counter {
        count: AtomicI64::new(0),
    });

    s.get("/", index);
    s.get("/increment", increment);
    s.get("/json", json_hello);
    s.post("/hello", receive_json);
    s.post("/db", check_database);

    s.serve("localhost", 8080).await
}
