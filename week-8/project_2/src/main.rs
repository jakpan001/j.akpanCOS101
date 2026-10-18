use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // A vector of tuples: (name, years of programming experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    println!("How many candidates are being interviewed?");
    let total: u32 = read_line().parse().expect("Invalid number");

    for n in 1..=total {
        println!("Candidate {} name:", n);
        let name = read_line();

        println!("Candidate {} years of programming experience:", n);
        let years: u32 = read_line().parse().expect("Invalid number");

        candidates.push((name, years));
    }

    if candidates.is_empty() {
        println!("No candidates to compare.");
        return;
    }

    // walk the vector, keeping the highest seen so far
    let mut best = &candidates[0];
    for candidate in &candidates {
        if candidate.1 > best.1 {
            best = candidate;
        }
    }

    // destructure the winning tuple
    let (name, years) = best;
    println!("\nHighest experience: {} with {} years.", name, years);
}