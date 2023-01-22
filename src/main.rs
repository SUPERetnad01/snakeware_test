mod api;
mod calculations;

use actix_web::{App, HttpServer};
use api::fibonacci_and_or_prime::check_fibonacci_and_or_prime;
use std::env;

static DEFAULT_ADDRESS: &str = "0.0.0.0";
static DEFAULT_PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = env::var("ADDRESS").unwrap_or(DEFAULT_ADDRESS.to_string());
    let port = env::var("PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse::<u16>()
        .unwrap();

    println!("the server runs on {address}:{port}");

    HttpServer::new(move || App::new().service(check_fibonacci_and_or_prime))
        .bind((address, port))? // TODO make environment variables
        .run()
        .await
}
