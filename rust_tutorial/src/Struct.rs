#![allow(unused)]
fn main() {

    struct Customer{
        name: String,
        address: String,
        balance: f32,
    }
    let mut bob = Customer{
        name: String::from("Anik"),
        address: String::from("Bukit Beruang"),
        balance: 100.00
    };

    println!("Address : {} \nBalance: {} \nName : {}\n\n", bob.address, bob.balance, bob.name);
    bob.address = String::from("Badda");
    println!("Address : {} \nBalance: {} \nName : {}", bob.address, bob.balance, bob.name);

}