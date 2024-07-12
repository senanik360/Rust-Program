#![allow(unused)]
fn main() {
    let mut arr_it = [1,2,3,4,5];
    for x in arr_it.iter(){
        print!("{} ",x);
    }
    println!();

    let mut iter1 =arr_it.iter();
    println!("1st : {:?}", iter1.next());
    println!("2nd : {:?}", iter1.next());
    println!("3rd : {:?}", iter1.next());
}