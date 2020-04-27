use std::vec;
fn main() {

    let mut _vec = vec![0; 5];
    
    first(&mut _vec);
}

fn first(mut v: &mut Vec<i32>) {
    println!("hello {}", v[2]);
    v[2] = 1;
    second(&mut v);
}

fn second(v: &mut Vec<i32>) {
    println!("bye {}", v[2]); 
}