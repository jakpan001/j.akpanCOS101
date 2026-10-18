use std::io;

// Read one trimmed line from the keyboard
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // One vector per column of the table; the same index is the same level
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec![
        "Intern", "Administrator", "Senior Administrator",
        "Office Manager", "Director", "CEO",
    ];
    let academic = vec![
        "-", "Research Assistant", "PhD Candidate",
        "Post-Doc Researcher", "Senior Lecturer", "Dean",
    ];
    let lawyer = vec![
        "Paralegal", "Junior Associate", "Associate",
        "Senior Associate 1-2", "Senior Associate 3-4", "Partner",
    ];
    let teacher = vec![
        "Placement", "Classroom Teacher", "Senior Teacher",
        "Leading Teacher", "Deputy Principal", "Principal",
    ];

    println!("=== Federal Public Service APS Level Checker ===");
    println!("Choose a career track:");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let choice: u32 = read_line().parse().expect("Please enter a number from 1 to 4");

    // pick the vector for the chosen track
    let roles = match choice {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid track.");
            return;
        }
    };

    println!("Enter the staff position (e.g. Associate):");
    let position = read_line();

    // walk the vector and compare, ignoring upper/lower case
    let mut found = false;
    for i in 0..roles.len() {
        if roles[i].to_lowercase() == position.to_lowercase() && roles[i] != "-" {
            println!("{} holds position {}", roles[i], levels[i]);
            found = true;
            break;
        }
    }

    if !found {
        println!("'{}' is not a valid position on that track.", position);
    }
}