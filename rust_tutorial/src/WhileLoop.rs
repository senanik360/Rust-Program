#![allow(unused)]
fn main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_i = 0;
    while loop_i < arr_2.len(){
        println!("Value : {}",arr_2[loop_i]);
        loop_i+=1;
    }
}