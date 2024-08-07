#![allow(unused)]
fn main() {
    let mut st1 = String::new();
    st1.push('A');
    println!("{}",st1);
    st1.push_str(" word");
    println!("{}",st1);
    for x in st1.split_whitespace(){
        println!("{}",x);
    }
    let st2=st1.replace("A", "Another");
    println!("{}",st2);

    // ------------------

    let st3 = String::from("h a k s b r d c d");
    let mut v1: Vec<char> = st3.chars().collect();

    v1.sort();
    v1.dedup(); //DELETE DUPLICATE
    for char in v1{
        println!("{}", char);
    }

    let st4 = String::from("Hello");
    let st5 = String::from(" World");

    println!("{}{}",st4,st5);

    let st6 = st4 + &st5; //st4 doesn't exist anymore but st5 still exist cz we used reference
    println!("{}",st6);


    for char in st6.bytes(){ //it will print unicode character
        println!("{}",char);
    }

}
