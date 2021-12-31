use std::collections::HashMap;

use salus::*;
use serde::{Deserialize, Serialize};

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
    let mut s = Salus::new();

    s.add(index {});
    s.serve("localhost", 8080).await
}
