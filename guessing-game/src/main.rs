use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Welcome to Guess the Number!");
    loop {
        println!("Please input your guess:");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read the line");
        // we are shadowing guess
        let guess:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("Please enter a number...");
                continue
            },
        };
        
        println!("Your guess: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            },
        }
    }
}


// String::new() creates and empty string
// std::io::stdin() creates a handle to the standard input. This handle has several methods
// .read_line() is one of this methods. It locks the handle and reads a line of input appending to the specified buffer
// the specified buffer must be a REFERENCE to a MUTABLE string (in our case the empty string)
// returns a Result type

// Buffer:  section of memory that is used to temporarily store some data that is being transferred from one place to another.

// Result<T, E> is an enum with the variants Ok(T) and Err(E)
// Ok(T) represents success and contains a value
// Err(E) represents error and contains an error
// the results must be handled: 
// you can use patter matching on Result for simples cases, 
// or you can use methods like .is_ok ir .is_err
// or you can assert success with the method .expect
// .expect will panic if the write fails, printing the msg provided
// this way you don't have to do any error handling which is convenient 
// when learning as a beginner (we will use this until we know more about enums, variants, pattern matching and error handling).  

// after importing the rand crate by adding it to the cargo.toml manualy or by typing cargo add rand in the terminal we can use it
// rand::thread_rng() is an assiciated function that returns a lazily-initialized thread-local random number generator, seeded by the system.
// this thread is intended to use in method chaining with methods
// rand::Rng is a trait that provide several methods to work with random numbers
// use rand::Rng will access to those methods, we are interested in the .gen_range() 
// .gen_range(low..high) generates a random value in the given range (exclusive)
// .gen_range(low..=high) high value inclusive

// std::cmp is a module within the standard library for comparing values
// std::cmp::Ordering is a enum with 3 variants: Less, Equal and Greater
// .cmp() method compares can be used to compare things
    // use std::cmp::Ordering;
    // assert_eq!(5.cmp(&10), Ordering::Less);
    // assert_eq!(10.cmp(&5), Ordering::Greater);
    // assert_eq!(5.cmp(&5), Ordering::Equal);

