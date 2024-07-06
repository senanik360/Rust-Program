#![allow(unused)]
fn say_hello() {
    println!("Hello.!");
}
fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}
fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}
fn get_sum_3(x: i32, y: i32) -> i32 {
    return (x + y)  // alternative way
}
fn get_sum_4(x: i32) -> (i32,i32) {
    return (x + 1, x+2); 
}
fn sum_list(list: &[i32]) -> i32 {
    let mut sum=0;
    for &val in list.iter(){
        sum+=&val;
    } 
    sum
}
fn main() {
    say_hello();
    get_sum(45, 50);
    println!("{}",get_sum_2(50, 50));
    println!("{}",get_sum_3(50, 50));

    let (val1, val2)=get_sum_4(5);
    println!("Numbers : {} {}", val1, val2);

    let num_list=vec![1,2,3,4,5,6];
    println!("Sum of List = {}", sum_list(&num_list));
}
