#![allow(unused)]

use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x:T, y:T) ->T{
    return x+y;
}


fn main(){
    println!(" 5 + 6 = {}", get_sum_gen(4,6));
    println!(" 5.5 + 4.5 = {}", get_sum_gen(5.5,4.5));
}
