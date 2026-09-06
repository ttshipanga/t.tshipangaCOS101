fn main() {
	let p: f64 = 210000.0;
	let r: f64 = 5.0;
	let n: i32 = 3;

	//depreciation
	let a = p * (1.0 - r / 100.0).powi(n);
	let de = a - p;
	println!("amount is {}", a);
	println!("depreciation is {}", de);
}