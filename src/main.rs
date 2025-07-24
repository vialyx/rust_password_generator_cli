use rand::{distributions::Alphanumeric, Rng};
use std::env;

fn main() {
    let length = env::args()
        .nth(1)
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(16); // default to 16 characters

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    println!("🔐 Generated password: {password}");
}