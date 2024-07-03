#![allow(unused)]

fn main(){
    let mut my_age = 47;
    let can_vote = if my_age >=18{
        true
    }else{
        false
    };
    println!("Can Vote : {}",can_vote);
}