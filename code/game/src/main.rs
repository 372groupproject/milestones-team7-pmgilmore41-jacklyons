use std::io;
use std::panic;
mod lib;

fn main() {
    let mut board = lib::connect4::Connect4Board::new();
    
    let mut win = 0;
    let mut buff = String::new();

    println!("Welcome to Connect 4 in Rust!");
    println!("Who's playing today? Enter your name below...");
    io::stdin().read_line(&mut buff).unwrap();
    let name = buff.trim();

    println!("Alright {}, ready to play? The rules are simple pick a column and drop your \ntoken and the token will fall to the bottom most row not filled.",name);
    println!("You are '1' and your oppenent is '2' and all empty spots are '0'");

    let mut col: usize = 0;
    while win != 1 {
        board.display();
        buff = String::new();
        io::stdin().read_line(&mut buff).unwrap();

        col = buff.trim().parse::<usize>().unwrap();

        win = board.drop(1, col);
    }
    
}
