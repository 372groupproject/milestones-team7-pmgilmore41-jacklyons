fn main(){
    let mut tup: (i32, String, char) = (86, String::from("John"), 'B');

    let (x, y, z) = tup;

    println!("GPA {} NAME {} LETTER GRADE {}", x,y,z);

    tup.0 = 89;

    println!("GPA change to {} from {}",tup.0, x);
}