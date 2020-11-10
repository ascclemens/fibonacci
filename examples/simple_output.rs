use fibonacci::Fibonacci;

fn main() {
	// Outputs `13`
	println!("{:1}", Fibonacci::<usize>::default().nth(5).unwrap());
}