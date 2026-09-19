// Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() {

    println!("Enter value of a:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to input");

    println!("Enter value of b:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter value of c:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    let d:f32 = b*b - 4.0*a*c;

    if d > 0.0 {
        let root1:f32 = (-b + d.sqrt()) / (2.0*a);
        let root2:f32 = (-b - d.sqrt()) / (2.0*a);

        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }
    else if d == 0.0 {
        let root:f32 = -b / (2.0*a);

        println!("There is exactly one real root: {}", root);
    }
    else {
        println!("There are no real roots");
    }
}