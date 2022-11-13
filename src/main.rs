// Fizz buzz in rust. Includes time of execution, fizz numbers & more.

use std::io;
use std::time::Instant;

fn main() {
    // Input number
    let mut input = String::new();
    println!("Enter a number count (recommended 1000000000, one bilion): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // Convert to u64
    let n: u64 = input.trim().parse().unwrap();

    // If the number is too big, it will take too long to execute. So we limit it to 1000000000.
    if n > 100000000000 {
        println!("Number is too big. This will take ages!");
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
        // Print percentage of progress, every 1%, "x" is repeat, "n" is total
        if x == 0 || x % (n / 100) == 0 {
            println!(
                "{}% | Fizzbuzz: {} | Fizz: {} | Buzz: {} | None: {}",
                x / (n / 100), fizzbuzzcount, fizzcount, buzzcount, normalcount
            );
        }
    }

    let duration = start.elapsed();

    // Print the result
    println!(
        "Took {:?} to calculate {} numbers of fizzbuzz", duration, n
    );
    println!(
        "Fizzbuzz: {}\nFizz: {}\nBuzz: {}\nNone: {}", fizzbuzzcount, fizzcount, buzzcount, normalcount
    );

    // Input nothing to keep the console open
    println!("Press enter to exit");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}