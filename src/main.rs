// SPDX-License-Identifier: MIT OR Apache-2.0

use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use axum::http::HeaderName;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    response::Json,
    routing::{get, post},
    Router,
};
use clap::Parser;
use lambda_http::tracing::{self, init_default_subscriber};
use lambda_http::{http::header, http::StatusCode, run, Error};
use tldr_lib::{KeySource, MyJwkEcKey, TangyLib};

#[derive(Clone)]
struct TangState {
    pub state: Arc<RwLock<TangyLib>>,
}

// Refer to https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/examples/http-axum-diesel/src/main.rs

const CONTENT_TYPE_JOSE_JSON: [(HeaderName, &str); 1] =
    [(header::CONTENT_TYPE, "application/jose+json")];

// #[get("/adv")]
#[axum::debug_handler]
async fn adv(State(tangy_state): State<TangState>) -> Result<impl IntoResponse, impl IntoResponse> {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(None) {
        Ok(a) => Ok((StatusCode::OK, CONTENT_TYPE_JOSE_JSON, a)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err((StatusCode::NOT_FOUND, CONTENT_TYPE_JOSE_JSON, e.to_string()))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CONTENT_TYPE_JOSE_JSON,
            // TODO Log and Redact internal error
            e.to_string(),
        )),
    }
}

// get("/adv/<skid>")
async fn adv_kid(
    State(tangy_state): State<TangState>,
    Path(skid): Path<String>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(Some(&skid)) {
        Ok(a) => Ok((StatusCode::OK, CONTENT_TYPE_JOSE_JSON, a)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err((StatusCode::NOT_FOUND, CONTENT_TYPE_JOSE_JSON, e.to_string()))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CONTENT_TYPE_JOSE_JSON,
            e.to_string(),
        )),
    }
}

// post("/rec/<kid>", data = "<data>")
async fn rec(
    State(tangy_state): State<TangState>,
    Path(kid): Path<String>,
    data: String,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.rec(kid.as_str(), data.as_str()) {
        Ok(a) => Ok((StatusCode::OK, CONTENT_TYPE_JOSE_JSON, a)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            Err((StatusCode::NOT_FOUND, CONTENT_TYPE_JOSE_JSON, e.to_string()))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CONTENT_TYPE_JOSE_JSON,
            e.to_string(),
        )),
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // TODO change path to a Uri to allow dispatching to various backends
    /// Database connection URI for Key Vault
    #[arg(short, long, env = "DATABASE_URI")]
    dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    init_default_subscriber();

    let tangy_state = TangState {
        state: Arc::new(RwLock::new(
            TangyLib::init(KeySource::LocalDir(&args.dir)).unwrap(),
        )),
    };

    let app = Router::new()
        .route("/adv/{skid}", get(adv_kid))
        .route("/adv", get(adv))
        .route("/rec/{kid}", post(rec))
        .with_state(tangy_state);

    run(app).await
}
