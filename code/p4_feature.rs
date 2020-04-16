

fn main(){
    let x = 10;
    println!("The factorial of {} is {}", x, factorial(x));
    println!("The largest divisor of {} is {}", x, largest_divisor(x));
}

fn factorial(mut x: i32) ->i32 {
    let mut result: i32 = 1;
    while x > 0 {
        result = result * x;
        x = x - 1;
    }
    result
}

fn largest_divisor(x: i32) -> i32 {
    let mut divisor: i32 =  1;
    let mut temp = divisor;
    while temp < x {
        if x % temp == 0 {
            divisor = temp
        }

        temp += 1;
    }

    divisor
}
