// Calculate annual incentive

use std::io;

fn main() {

    println!("Are you experienced? (yes/no)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience = input1.trim();

    println!("Enter your age:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i32 = input2.trim().parse().expect("Failed to input");

    if experience == "yes" && age >= 40 {
        println!("Annual incentive: N1,560,000");
    }
    else if experience == "yes" && age >= 30 && age <= 39 {
        println!("Annual incentive: N1,480,000");
    }
    else if experience == "yes" && age < 28 {
        println!("Annual incentive: N1,300,000");
    }
    else if experience == "no" {
        println!("Annual incentive: N100,000");
    }
}