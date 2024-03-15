use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=10);

        println!("I'm thinking of a number between 1 and 10!");
        println!("What do you think it is?");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read...");

        let guess: u32 = guess.trim().parse().expect("Please use a number.\n");
        println!("You guessed {guess}");
        println!("Secret number {secret_number}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Greater => println!("Too high!"),
        }
        println!("Let's play again!");
    }
}