// Fizz buzz in rust. Includes time of execution, fizz numbers & more.

use std::io;
use std::time::Instant;

fn main() {
    loop {
        println!("Enter a number count (recommended 1000000000, one bilion): ");
        
        // Get user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Invalid input, please enter a number!");
                return;
            }
        }

        // Convert to u64
        let n: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // If the number is too big, it will take too long to execute. So we limit it to 1000000000.
        if n > 100000000000 {
            println!("Number is too big. This would take ages!");
            return;
        }

        let mut fizzcount: u64 = 0;
        let mut buzzcount: u64 = 0;
        let mut fizzbuzzcount: u64 = 0;
        let mut normalcount: u64 = 0;

        // Start the timer
        let start = Instant::now();

        for x in 1..n {
            match (x % 3, x % 5) {
                (0, 0) => {
                    //println!("FizzBuzz");
                    fizzbuzzcount += 1;
                }
                (0, _) => {
                    //println!("Fizz");
                    fizzcount += 1;
                }
                (_, 0) => {
                    //println!("Buzz");
                    buzzcount += 1;
                }
                (_, _) => {
                    //println!("{}", x);
                    normalcount += 1;
                }
            }

            // TODO: Print percentage of execution, make it work with numbers below 100
            // No idea how to do this, but it would be cool to see the progress of the program.
        }

        let duration = start.elapsed();

        // Print the result
        println!("Took {:?} to calculate {} numbers of fizzbuzz", duration, n);
        println!(
            "Fizzbuzz: {}\nFizz: {}\nBuzz: {}\nNone: {}",
            fizzbuzzcount, fizzcount, buzzcount, normalcount
        );

        // Input nothing to keep the console open
        println!("Press enter to exit or enter anything to continue (better implementation coming in v5.0)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "" {
            break;
        }
    }
}
