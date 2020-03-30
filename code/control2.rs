fn main(){
	let x = false;
	let y = true;

	if x {
		println!("TRUE");
	}
	else {
		if y {
			println!("TRUE");
		}
		else {
			println!("False");
		}
	}
}