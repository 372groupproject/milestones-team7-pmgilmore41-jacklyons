use std::vec;
fn main() {

    let mut _vec = vec![0; 5];
    
    first(&mut _vec);

    //invalid borrow
    //let copy = _vec;

    //valid
    let mut copy = &_vec;

    //cannot change as a reference type
    //copy[0] = 5;

    println!("{}\n",getmax(copy));

    for elem in _vec.iter() {
        println!("{}", elem);
    }
}

fn first(mut v: &mut Vec<i32>) {
    println!("hello {}", v[2]);
    v[2] = 1;
    second(&mut v);
}

fn second(v: &mut Vec<i32>) {
    println!("bye {}", v[2]); 
}


//must be passed a reference now 
fn getmax(v: &Vec<i32>) -> i32{
    let mut max: i32 = 0;

    for elem in v.iter(){
        if *elem > max {
            max = *elem;
        }
    }

    max
}