fn main(){
    let one: i32 = 56;

    let two: f64 = 52.6;

    //IMPORTANT you cannot add ints to floats but you can cast to avoid this.
    let combo: f64 = (one as f64)+two;

    println!("{}", combo);
}