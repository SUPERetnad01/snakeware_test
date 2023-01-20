use crate::calculations::{fibonacci::is_in_fibonacci_sequence, prime::is_prime};
use actix_web::{get, web::Path, HttpResponse};

use serde::Serialize;
#[derive(Debug, Serialize)]
struct FibonacciOrPrimeResult {
    is_in_fibonacci: bool,
    is_prime_number: bool,
}

// checks if a number is in the sequence of fibonacci and if it is a prime number
#[get("/{number}")]
pub async fn check_fibonacci_and_or_prime(path: Path<u64>) -> HttpResponse {
    let input_number = path.into_inner();
    let is_in_fibonacci = match is_in_fibonacci_sequence(input_number.clone(), None) {
        Ok(r) => r,
        Err(err) => return HttpResponse::BadRequest().body(err),
    };

    let result = FibonacciOrPrimeResult {
        is_in_fibonacci: is_in_fibonacci,
        is_prime_number: is_prime(input_number),
    };
    return HttpResponse::Ok().json(result);
}
