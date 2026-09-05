fn main() {
	//Sales amounts
	let sales = [
	450_000.00,
	1_500_000.00,
	750_000.00,
	2_850_000.00,
	250_000.00,
	];
	//Calculate the sum
	let sum: f64 = sales.iter().sum();
	//Calculate the average
	let average = sum / sales.len() as f64;
	println!("Total sales: N{}", sum);
	print!("Average Sales: N{}", average);
}