use std::io;

fn main() {
    // print the menu
    println!("Good day Ma/Sir, Welcome to K.J Restaurant");
    println!("What can we offer you Today?");
    println!("Take a look at our menu and place your order");
    println!("              K.J Restaurant Menu           ");
    println!("{:<6} {:<30} {:>8}", "Codes", "Food", "Prices");
    println!("{:<6} {:<30} {:>8}", "P", "Poundo Yam / Edikang Ikong", "N3,200");
    println!("{:<6} {:<30} {:>8}", "F", "Fried Rice & Chicken", "N3,000");
    println!("{:<6} {:<30} {:>8}", "A", "Amala & Ewedu Soup", "N2,500");
    println!("{:<6} {:<30} {:>8}", "E", "Eba & Egusi Soup", "N2,000");
    println!("{:<6} {:<30} {:>8}", "W", "White Rice & Stew", "N2,500");

    let mut total: u32 = 0;

    loop {
        // ask for the food code
        let mut choice = String::new();
        println!("Enter food code (or D when done): ");
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_uppercase();

        if choice == "D" {
            break;
        }

        // decide the price
        let price: u32 = match choice.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid food code! Try again.");
                continue;
            }
        };

        // ask for the quantity
        let mut qty_input = String::new();
        println!("Enter quantity: ");
        io::stdin().read_line(&mut qty_input).expect("Failed to read input");

        let quantity: u32 = match qty_input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        total += price * quantity;
        println!("Current total: N{}", total);
    } // <-- the loop now ends here

    // apply the discount after the loop
    let mut final_total = total as f64;

    if total > 10000 {
        let discount = final_total * 0.05;
        println!("Discount (5%): N{:.2}", discount);
        final_total -= discount;
    }

    println!("Total to pay: N{:.2}", final_total);
    println!("Do Have a great day and Enjoy your meal");
}