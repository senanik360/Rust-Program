#![allow(unused)]

use std::cmp::Ordering;
fn main() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("OK!"),
        40 | 50 => println!("OK!"),
        55..=i32::MAX => println!("OK!"),
        _ => println!("Not Ok!"), //match everything else
    };

    let myAge=18;
    let votingAge=18;
    match myAge.cmp(&votingAge){
       Ordering::Less =>println!("Teenage!"),
        Ordering::Equal => println!("Border Line"),
        Ordering::Greater => println!("Adult."),       
    };
}
