
fn main() {
    println!("Hello, world!");
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    let _ = std::io::stdin().read_line(&mut guess);

    println!("You guessed {}", guess);

}
