
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, newtype_maths::Add)]
struct Metre(i32);

fn main() {
	let a = Metre(1);
	let b = Metre(2);
	let c: Metre = a + b;
	println!("Hello, world!");
}
