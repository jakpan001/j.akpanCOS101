struct Laptop{
    brand: String,
    price: u32
}
// The method that cost a purchase
impl Laptop {
    fn cost(&self)->u32{
        self.price * 3
    }
}
fn main() {
    let brand_1 = Laptop {
        brand: String::from("HP"),
        price: 650_000
    };
    let brand_2 = Laptop{
        brand: String::from("IBM"),
        price: 755_000
    };
    let brand_3 = Laptop{
        brand: String::from("Toshiba"),
        price: 550_000
    };
    let brand_4 = Laptop{
        brand: String::from("Dell"),
        price: 850_000
    };
    // THE COST FOR 3 LAPTOPS
    let cost_1 = brand_1.cost();
    let cost_2 = brand_2.cost();
    let cost_3 = brand_3.cost();
    let cost_4 = brand_4.cost();
    // Printing the Cost for each brand
    println!("The cost for 3 {} Laptops is: N{}",brand_1.brand,cost_1);
    println!("The cost for 3 {} Laptops is: N{}",brand_2.brand,cost_2);
    println!("The cost for 3 {} Laptops is: N{}",brand_3.brand,cost_3);
    println!("The cost for 3 {} Laptops is: N{}",brand_4.brand,cost_4);

    //The Cost for all the 4 brands
    let total_cost = cost_1 + cost_2 + cost_3 + cost_4;
    println!("The total cost is N{}",total_cost); 

}