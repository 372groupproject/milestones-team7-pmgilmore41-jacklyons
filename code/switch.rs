fn main() {
	let x = 2;
	
	let mut y = 0;
	match x {
		1 => {println!("x == 1"); y = 1;},
		2 => {println!("x == 2"); y = 2;},
		_ => println!("x is undefined in our program!"),
	}

	println!("y == {:?}", y);
}