fn main(){
    let y = 5;

    let x = if y < 5 {
        y+5
    } 
    else if y > 5 {
        y-5
    }
    else {
        y
    };

    println!("x's value is {:?}", x);
}