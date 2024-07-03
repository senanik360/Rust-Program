// Random.rs

use rand::Rng; // Import the Rng trait from the rand crate

fn main() {
    let random_num = rand::thread_rng().gen_range(1..=100); // Generate a random number between 1 and 100 (inclusive)
    println!("Random number: {}", random_num);
}
