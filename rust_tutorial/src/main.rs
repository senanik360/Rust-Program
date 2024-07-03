#![allow(unused)] //for get rid of some error when i have some variables that are declared but not being used

mod DataTypes;
pub mod Math;
//mod Random;
mod constant;
pub mod If;
mod TerneryOperator;
mod Match;
mod Array;

//extern crate rand;
use std::io;
//use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
    println!("What is your name ?");
    let mut name: String = String::new();
    let greeting: &str = "Nice to Meet You.";
    let read_line = io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello, {}! {}", name.trim_end(), greeting);

}
