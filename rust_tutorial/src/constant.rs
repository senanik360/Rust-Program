#![allow(unused)]
fn main(){
    const  ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age="47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    //variables with same name bu different data types called shadowing 
    age = age + 1;
    println!("I'm {} and i want ${}", age, ONE_MIL);

}