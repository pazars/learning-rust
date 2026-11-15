use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let number = rand::thread_rng().gen_range(1..=100);
    let mut cnt: u8 = 1;

    // In a terminal stdout flushes on newline
    println!("debug: The secret number is {number}");

    // For a regular print we flush explicitly
    // unwrap handles the Result, panics on error
    print!("Input a number [1-100] or q to quit: ");
    io::stdout().flush().unwrap();
 
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("read_line failed");

        if guess.trim() == "q" {
            break;
        }

        // Shadowing for string conversion to int
        // trim gets rid of whitespace and returns \n \r\n
        // parse performs the conversion
        // in this example 8 bits enough for 1..100 range
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&number) {
            Ordering::Greater => println!("Too high"),
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }

        if cnt == 5 {
            println!("Out of guesses. The number was {number}");
            break;
        }
        
        println!("[{cnt}/5] Try again: ");
        io::stdout().flush().unwrap();

        cnt += 1;
    }
}
