fn main() {
	let p: f64 = 210000.0;
	let r: f64 = 5;
	let t: f64 = 3;

	let A = p*(1 - (r/100)).powf(t);
	println!("Amount is{}", A);
}