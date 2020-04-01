fn main(){
    let mut num = 6;
    let result = loop {
        num = num/2;

        if num < 2 {
            break num*2;
        }
    };

    println!("The value of num before exiting was {:?}", result);
}