fn main() {
	let tosh:f64 = 450_000.0;
	let mac:f64 = 1_500_000.0;
	let hp:f64 = 750_000.0;
	let dell:f64 = 2_850_000.0;
	let acer:f64 = 250_000.0;
	let sum:f64 = 10.0;

	//sum of items
	let tot= tosh + mac + hp + dell+ acer;
	println!("The sum of items is {}", tot);

	//average
	let a = tot / sum;
	println!("The average is {}",a);
}