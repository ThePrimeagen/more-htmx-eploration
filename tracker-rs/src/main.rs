mod templates;
use axum::{routing::get, Router};
use templates::{Index, Timing};
//use tokio::sync::Mutex;

/*
lazy_static::lazy_static! {
    static ref TIMINGS: Mutex<Vec<Timing>> = Mutex::new(Vec::new());
}
*/

async fn index() -> Index {
    return Index {
        timings: vec![Timing {
            timing_type: "movement",
            start: 0,
            stop: 25,
            id: 1,
        }, Timing {
            timing_type: "movement",
            start: 75,
            stop: 120,
            id: 2,
        }]
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
