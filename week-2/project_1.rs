fn main() {
	let p:f64 = 520_000_000.00;
	let r:f64 = 10.0;
	let t:f64 = 5.0;

	// Compund Interest
	let a = p * ( 1.0 + (r/100.0)) * t;
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interst is {}", ci);

}