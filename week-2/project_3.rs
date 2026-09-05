fn main() {
    let principal = 210_000.0;
    let rate = 5.0;
    let years = 3.0;

    // Depreciation Formula

    let value = principal * (1.0 - rate / 100.0);

    println!("Original Value: ₦{:.2}", principal);
    println!("Depreciation Rate: {}%", rate);
    println!("Number of Years: {}", years);
    println!("Value after 3 years: ₦{:.2}", value);
}