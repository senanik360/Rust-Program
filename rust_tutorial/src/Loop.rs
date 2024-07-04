#![allow(unused)]
fn main() {
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_i = 0;
    println!("Hello");

    loop {
        if arr_2[loop_i] % 2 == 0 {
            loop_i += 1;
            continue;
        }
        if arr_2[loop_i] == 9 {
            break;
        } 
        println!("Val : {}", arr_2[loop_i]);
        loop_i += 1;
    }
   
   
}

