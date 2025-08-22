use rand::Rng; //its for random number
use std::cmp::Ordering;
use std::io; //for standard input output

fn main() {
    println!("( ======  Guess the Number!  ======  ");

    //generating a secret number
    let secret_number: u32 = rand::rng().random_range(1..=100);
    print!("Generated Secret Number {secret_number}");

    println!("\nPlease Input your Guess?: ");

    let mut guess: String = String::new();

    //taking input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Failed to parse!"); //here you see we store in same variable its shadowing mtlb k same variable again declare kr skty ab is line k baad ye walla ho gaa not previous one

    //compare
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You Won Congrats!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Less => println!("Too Small!"),
    }
}
