use std::net::SocketAddr;

use salus::*;

async fn handle() -> String {
    "Hello, world!".to_string()
}

#[tokio::main]
async fn main() {
    use http::Method;
    let mut s = Salus::new();

    s.add_route("/".into(), Method::GET, handle);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    s.serve(addr).await;
}
