// Name and Age Guesser

use std::io;

fn main() {
    println!("Enter your lovely name:");
    let mut input_1 = String::new();
    io::stdin().read_line(&mut input_1).expect("invalid input!");
    
    println!("Enter Your age:");
    let mut input_2 = String::new();
    io::stdin().read_line(&mut input_2).expect("Wrong Data Type");
    
    let age:u8 = input_2.trim().parse().expect("the num you inputted isn't between 0-255");
    if age > 1 {
    println!("Hey, {} You are {} years old",input_1.trim(),input_2.trim());
        }
    else {
        println!("Hey, {}You are {}year old",input_1,input_2);
    }
}