// Fizz buzz in rust. Includes time of execution, fizz numbers & more.

use std::io::{self, Write};
use std::time::Instant;

struct Count {
    fizz: u64,
    buzz: u64,
    fizzbuzz: u64,
    normal: u64,
}

fn calculate_n(max: u64) -> Count {
    let mut count = Count {
        fizz: 0,
        buzz: 0,
        fizzbuzz: 0,
        normal: 0,
    };

    for x in 1..=max {
        match (x % 3, x % 5) {
            (0, 0) => {
                //println!("FizzBuzz");
                count.fizzbuzz += 1;
            }
            (0, _) => {
                //println!("Fizz");
                count.fizz += 1;
            }
            (_, 0) => {
                //println!("Buzz");
                count.buzz += 1;
            }
            (_, _) => {
                //println!("{}", x);
                count.normal += 1;
            }
        }
        // TODO: Add a way to print the numbers ever percentage of the way through.
    }
    count
}

fn main() {
    loop {
        let mut input = String::new();
        // Input number
        print!("Enter a number count or press enter to exit: ");
        io::stdout().flush().unwrap();

        // Get user input
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => {
                println!("Error reading input.");
                continue;
            }
        }

        // Check if user wants to exit
        if input.trim().is_empty() {
            break;
        }

        // Convert to u64
        let max: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Check if the number is too big
        if max > 1000000000000 {
            println!("Number is too big. This would take ages! Try a number around 1000000000 (9 zeros).");
            continue;
        }

        // Clear input
        input.clear();

        // Start timer
        let start = Instant::now();

        // Start calculation function
        let count = calculate_n(max);

        // End timer
        let duration = start.elapsed();

        // Calculate the rate of calculating
        let rate = max as f64 / duration.as_secs_f64();

        // Print results
        println!(
            "Took {:?} to calculate {} numbers of fizzbuzz.\nAverage rate per second: ~{}",
            duration,
            max,
            rate.round()
        );
        println!(
            "Fizzbuzz: {}\nFizz: {}\nBuzz: {}\nNone: {}",
            count.fizzbuzz, count.fizz, count.buzz, count.normal
        );
    }
}
