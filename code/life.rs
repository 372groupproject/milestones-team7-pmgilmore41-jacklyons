fn main() {
    
    let mut x = [1,2,3,4];
    let y = x[1];
    x[1] = 3;

    println!("{}", y);


    let s1 = String::from("Test");
    let y = &s1; //NOTE CANNOT ASSIGN TO JUST s1 BC value moves

    println!("Test String {}", s1);
}