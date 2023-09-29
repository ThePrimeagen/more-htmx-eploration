mod templates;
use axum::{routing::get, Router};
use templates::Index;
//use tokio::sync::Mutex;

/*
lazy_static::lazy_static! {
    static ref TIMINGS: Mutex<Vec<Timing>> = Mutex::new(Vec::new());
}
*/

async fn index() -> Index {
    return Index {
        timings: vec![]
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
