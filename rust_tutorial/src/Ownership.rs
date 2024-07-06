#![allow(unused)]

fn change_string(name: &mut String){
    name.push_str("Be Happy Always");
    println!("Message : {}", name);
}

fn main(){
 let str1 = String::from("World");
 //let str2 = str1; // it will show an error
 let str2 = str1.clone();
 let str3 = &str1;
 
 println!("Hello {}",str1);
 println!("Hello {}",str1);
 let mut name = String::from("Anik, ");
 change_string(&mut name);
}


//Stack : stores values in a last in first out format.Data on the stack must have a defined fixed size

// Heap : whe n putting data on the heap you request a certain amount of space. the OS finds space available and returns an address for that space called a pointer. 
//Rules 
    // 1. Each value has a variable that's called its owner
    // 2. There is only one owner at a time
    // 3. When the owner goes out of scope the value disappears.