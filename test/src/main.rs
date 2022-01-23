
use newtype_maths::{Add, AddAssign, Neg};

#[derive(Debug, Add, AddAssign, Neg)]
struct Metre(i32);

fn main() {
	{
		let a = Metre(1);
		let b = Metre(2);
		let c: Metre = a + b;
		dbg!(c);
	}
	{
		let mut a = Metre(1);
		let b = Metre(2);
		a += b;
		dbg!(a);
	}
	{
		let a = Metre(1);
		let b = -a;
		dbg!(b);
	}
}
