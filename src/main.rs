use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!(" ");
    println!("I have chosen a number between 1 and 100!");
    println!("Guess which number I have chosen...");

    let secret_number = rand::thread_rng().gen_range(1..100);
    
    let mut attempts: u8 = 1;

    loop {
        println!(" ");
        println!("Attempt {}: ", &attempts);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("You guessed: {}. That is lower than my number.", guess);
                attempts = attempts + 1;
            },
            Ordering::Greater => {
                println!("You guessed: {}. That is higher than my number.", guess);
                attempts = attempts + 1;
            },
            Ordering::Equal => {
                println!("Congratulations, you are correct!");
                println!("You guessed the number was {} in only {} attempts!", &secret_number, &attempts);
                break;
            }
        }
    }
}

