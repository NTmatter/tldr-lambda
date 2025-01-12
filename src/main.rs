// SPDX-License-Identifier: MIT OR Apache-2.0

#[macro_use]
extern crate rocket;

use std::future::Future;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use clap::Parser;
use lambda_http::{Body, Error, http::Uri, Request, Response, run, service_fn, tracing, IntoResponse, Service};
use lambda_http::http::StatusCode;
use lambda_http::tracing::init_default_subscriber;
use rocket::http::{ContentType, Status};
use rocket::State;
use tldr_lib::{KeySource, TangyLib};

struct TangState {
    pub state: RwLock<TangyLib>,
}

#[get("/adv")]
fn adv(tangy_state: &State<TangState>) -> (Status, (ContentType, Option<String>)) {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(None) {
        Ok(a) => (
            Status::Ok,
            (ContentType::new("application", "jose+json"), Some(a)),
        ),

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            Status::NotFound,
            (ContentType::new("application", "jose+json"), None),
        ),

        Err(_) => (
            Status::InternalServerError,
            (ContentType::new("application", "jose+json"), None),
        ),
    }
}

#[get("/adv/<skid>")]
fn adv_kid(skid: &str, tangy_state: &State<TangState>) -> (Status, (ContentType, Option<String>)) {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.adv(Some(skid)) {
        Ok(a) => (
            Status::Ok,
            (ContentType::new("application", "jose+json"), Some(a)),
        ),

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            Status::NotFound,
            (ContentType::new("application", "jose+json"), None),
        ),
        Err(_) => (
            Status::InternalServerError,
            (ContentType::new("application", "jose+json"), None),
        ),
    }
}

#[post("/rec/<kid>", data = "<data>")]
fn rec(
    kid: &str,
    data: &str,
    tangy_state: &State<TangState>,
) -> (Status, (ContentType, Option<String>)) {
    let tangy = tangy_state.state.read().unwrap();

    match tangy.rec(kid, data) {
        Ok(r) => (
            Status::Ok,
            (ContentType::new("application", "jwk+json"), Some(r)),
        ),

        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (
            Status::NotFound,
            (ContentType::new("application", "jose+json"), None),
        ),
        Err(_) => (
            Status::InternalServerError,
            (ContentType::new("application", "jose+json"), None),
        ),
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

async fn function_handler(event: Request) -> Result<(StatusCode, String), Error> {
    // Identify path and components
    let uri = event.uri();
    let mut components = uri.path().split('/');

    // Consume prefix if present.
    // for _ in self.prefix {
    //    components.next();
    // }
    let endpoint = components.next();
    let arg = components.next();

    match (endpoint, arg) {
        (Some("adv"), None) => {},
        (Some("adv"), Some(_skid)) => {},
        (Some("rec"), None) => {},
        (_, _) => todo!("404 Error")
    }

    todo!()
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    init_default_subscriber();

    let tangy_state = TangState {
        state: RwLock::new(TangyLib::init(KeySource::LocalDir(&args.dir)).unwrap()),
    };

    let foo = Arc::new(RwLock::new(()));
    run(service_fn(function_handler)).await?;

    Ok(())
}
