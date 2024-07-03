#![allow(unused)]

fn main()
{
    let num1: f32=0.111111111111111;
    println!("f32 : {}",num1+1.111111111111111);
    let num2: f64=0.111111111111111;
    println!("f64 : {}",num2+1.111111111111111);

    let num3: u32 = 5;
    let num4: u32 = 4;
    println!("5 + 4 : {}",num3+num4);
    println!("5 - 4 : {}",num3-num4);
    println!("5 / 4 : {}",num3/num4);
    println!("5 * 4 : {}",num3*num4);
    println!("5 % 4 : {}",num3%num4);
}