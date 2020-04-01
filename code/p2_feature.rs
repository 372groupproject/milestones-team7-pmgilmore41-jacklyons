use std::io;

fn main() {

    println!("Lets play a guessing game!");

    let guess_list = [5, 10, 17, 18, 1];

    println!("Guess any of the 5 numbers in the secret list and you Win!! But you only have five tries.");

    let mut guess = 1;
    let mut buff = String::new();
    let mut curr: i32 = 0;

    while guess != 5 {
        buff = String::new();
        println!("Guess number {}", guess);
        
        
        io::stdin().read_line(&mut buff).unwrap();
        curr = buff.trim().parse::<i32>().unwrap();
        let mut truth = false;

        for elem in guess_list.iter() {
            let e = *elem as i32;
            if e == curr {
                truth = true;
                break;
            }
        }

        if truth {
            println!("YOU GOT ONE!");
            break;
        }
        else {
            println!("TRY AGAIN!");
        }
        guess += 1;
    }

    match guess {
        1 => println!("Master guesser!"),
        2 => println!("Pretty darn good."),
        3 => println!("Well done."),
        4 => println!("Cutting it close huh?"),
        5 => println!("Last try... you got lucky!"),
        _ => println!("TOO BAD. Reload the progam and try again!")
    }
}