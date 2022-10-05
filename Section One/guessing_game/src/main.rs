use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is {}",secret_num);
    loop {
    println!("Enter your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed {guess}");
    match guess.cmp(&secret_num){
        Ordering::Less => println!("Too less!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
             println!("Correct guess!");
             break;},
    }};

    
}