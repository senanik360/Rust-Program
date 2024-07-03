#![allow(unused)]

//extern crate rand; // Optional in Rust 2018 edition and onwards

use rand::Rng; // Import the Rng trait from the rand crate
fn main(){
let random_num =  rand::thread_rng().gen_range(1..101);
}