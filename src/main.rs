use std::io;
use std::time::Instant;

// public variable
pub static mut FIZZCOUNT: u64 = 0;
pub static mut BUZZCOUNT: u64 = 0;
pub static mut FIZZBUZZCOUNT: u64 = 0;
pub static mut NORMALCOUNT: u64 = 0;

fn main() {
    let mut input = String::new();
    println!("Enter a number count (recommended 1000000000, one bilion): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n: u64 = input.trim().parse().unwrap();
    if n > 100000000000 {
        println!("Number is too big. This will take ages!");
        return;
    }

    // Measure the time it takes to run the function
    let start = Instant::now();
    let result = fizzbuzz_to(n);
    let duration = start.elapsed();
    println!(
        "Took {:?} to calculate {} numbers of fizzbuzz",
        duration, result
    );
    println!(
        "Fizz: {}, Buzz: {}, Fizzbuzz: {}, None: {}",
        unsafe { FIZZCOUNT },
        unsafe { BUZZCOUNT },
        unsafe { FIZZBUZZCOUNT },
        unsafe { NORMALCOUNT }
    );
    println!("Press enter to exit");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u64, rhs: u64) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u64) {
    if is_divisible_by(n, 15) {
        unsafe {
            FIZZBUZZCOUNT += 1;
        }
    //        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        unsafe {
            FIZZCOUNT += 1;
        }
    //        println!("fizz");
    } else if is_divisible_by(n, 5) {
        unsafe {
            BUZZCOUNT += 1;
        }
    //        println!("buzz");
    } else {
        unsafe {
            NORMALCOUNT += 1;
        }
        //        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u64) -> u64 {
    for repeat in 1..=n {
        fizzbuzz(repeat);
        // Print percentage of progress, every 1%, current is repeat, total is n
        if repeat == 0 || repeat % (n / 100) == 0 {
            println!(
                "{}% | Fizzbuzz: {} | Fizz: {} | Buzz: {} | None: {}",
                repeat / (n / 100),
                unsafe { FIZZBUZZCOUNT },
                unsafe { FIZZCOUNT },
                unsafe { BUZZCOUNT },
                unsafe { NORMALCOUNT }
            );
        }
    }
    n
}
