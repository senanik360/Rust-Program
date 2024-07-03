#![allow(unused)]

fn main(){
    let age=8;
    if(age>=1) && (age <=18){
        println!("Oky!");
    }else if(age==21) || (age ==50){
        println!("Oky!");
    }else if age>65{
        println!("Oky!");
    }else {
        println!("Not Oky!");
    }
}