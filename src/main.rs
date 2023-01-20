mod api;
mod calculations;

use actix_web::{App, HttpServer};
use api::fibonacci_and_or_prime::check_fibonacci_and_or_prime;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    println!("the server runs on {address}:{port}");
    HttpServer::new(move || App::new().service(check_fibonacci_and_or_prime))
        .bind((address, port))? // TODO make environment variables
        .run()
        .await
}
