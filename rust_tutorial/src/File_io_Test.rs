#![allow(unused)]

use std::{
    fs::File,
    io::{BufRead, BufReader, Write}, // Added Write to the import
};
fn main() {
    let path = "Lines.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem Creating File: {:?}", error);
        }
    };

    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let output2 = File::create("rand.txt");
    let output2 = match output2{
        Ok(file) => file,
        Err(error)=> match error.kind(){
            ErrorKind =>match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file: {:?} ", error),

            },
            _other_error =>panic!("Problem opening file: {:?}", error),
        },
    };
}
