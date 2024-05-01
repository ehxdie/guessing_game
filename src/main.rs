use std::io;
// Pulling from the rand crate a functions and methods associated with the Rng trait
use rand::Rng;

// Comparison functions
use std::cmp::Ordering;

fn main() {
    println!("Input your number");
    
    loop {
    // Variable holding the user input string
    let mut input = String::new();

   

    // Reading user input operation
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // Printing out user input
    println!("You inputted the number {input}");

    // Generating the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {}", secret_number);

    // Converting the guess to an int
    
    let input: u32 = input.trim().parse().expect("Please type a number!"); 

    // Handling comparisons between the guess and the secret number
    // match input {
    //     secret_number => println!("You win"),
    //     _ => println!("You lose"),

    // }
    match input.cmp(&secret_number) {
        Ordering::Less => println!("less than secret number"),
        Ordering::Greater => println!("greater than secret number"),
        Ordering::Equal => {
            println!("equal to secret number, YOU WIN !!"); 
            break;
        }

    }

}
}

