use serde::{Deserialize, Serialize};

/// # Primality tester
///
/// ```
/// use protohackers::prime_time::is_prime;
/// assert_eq!(is_prime(0), false);
/// assert_eq!(is_prime(1), false);
/// assert_eq!(is_prime(2), true);
/// assert_eq!(is_prime(3), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(5), true);
/// assert_eq!(is_prime(6), false);
/// assert_eq!(is_prime(7), true);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    let cap = (n as f64).sqrt() as u64 + 1;
    !(2..cap).any(|i| n % i == 0)
}

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub number: serde_json::Number,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub method: String,
    pub prime: bool,
}
