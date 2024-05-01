use std::io;
use rand::Rng;

fn main() {
    println!("Input your number");
    
    // Variable holding the user input string
    let mut input = String::new();

    // Reading user input operation
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    // Printing out user input
    println!("You inputted the number {input}");
    

   

}

