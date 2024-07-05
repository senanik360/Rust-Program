#![allow(unused)]
fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

   // println!("{}",vec2);
   for x in vec2.iter(){
    println!("{}",x);
   }
   vec2.push(5);
   for x in vec2.iter(){
    println!("{}",x);
   }
   println!("--------");
   println!("1st value : {}", vec2[0]);

   let second=&vec2[1];

   match vec2.get(1){
    Some(second) =>println!("2nd value : {}",second),
    None => println!("No 2nd Value"),

   }
   println!("Vector Length : {}",vec2.len());
   for i in &mut vec2{
    *i *=2;
   }
    println!("Pop : {:?}",vec2.pop());

   for x in vec2{
    println!("{}",x);
   }
   //println!("Pop : {:?}",vec2.pop());

}