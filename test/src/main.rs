use newtype_maths::{Add, AddAssign, Neg, Index, IndexMut, From};

#[derive(Copy, Clone, Debug, Add, AddAssign, Neg, From)]
struct Metre(i32);

#[derive(Copy, Clone, Debug, Add, AddAssign, Neg)]
struct Kilogramme(i32);

#[derive(Index, IndexMut)]
struct List(Vec<i32>);

fn main() {
	let mut a = Metre(1);
	let b = Metre(2);
	let c = a + b;

	dbg!(a);

	a += c;

	dbg!(a, b, c);

	let mut list = List(vec![1,2,3]);
	let one = list[2];
	dbg!(one);

	list[2] = 4;
	let two = list[2];
	dbg!(two);

	let m = Metre(1);
	let kg = Kilogramme(2);
	// let nonsense = m + kg;

	// dbg!(m, kg, nonsense);
	
	let d: Metre = 1i32.into();
	dbg!(d);
}

