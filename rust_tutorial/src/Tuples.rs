#![allow(unused)]
fn main() {
    let my_tuple =(47,"Derek".to_string(), 50_00.00);
    

    println!("Name: {}",my_tuple.1);

     let(v1, v2, v3)=my_tuple;
    println!("Age: {}",v1); 
}
