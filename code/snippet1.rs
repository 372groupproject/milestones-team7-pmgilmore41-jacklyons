use std::io;

fn main() {
    println!("What would you like to say to me? I'll echo it back!!");

    let mut user = String::new();

    io::stdin().read_line(&mut user).expect("Failed to read user.");

    println!("You said: {}", user);
}