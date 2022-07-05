use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        println!("Enter your secret number: ");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        println!("You guessed: {guess}");
    
        let secret_num = rand::thread_rng().gen_range(1..=5);
        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect number!");
                continue;
            }
        };
    
        match guess_int.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
