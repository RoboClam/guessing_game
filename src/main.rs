use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Guess the number between 1 and 10!");
    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read...");

    println!("You guessed {guess}");
    println!("Secret number {secret_number}");

}
