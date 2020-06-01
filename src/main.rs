//! The 3n+1 Problem
//!
//! This program solves the 3n+1 Problem, Problem #100 from onlinejudge.org. Input is read from
//! stdin and the results printed to stdout. Input is expected to be a pair of positive integers on
//! a single line, separated by any amount of whitespace. The output is the two input integers,
//! followed by the maximum cycle length of the 3n+1 algorithm for each integer between them
//! (inclusive).
//!
//! The program will continue to accept input until a blank line is encountered, after which it
//! will terminate.

use threen::{read_args, InputError, ThreeN};

/// This is the program's main loop.
fn main() -> Result<(), InputError> {
    // Loop forever, or until we break out
    loop {
        match read_args() {
            // Received no arguments, break out of our loop
            Ok(None) => break,
            // Go a pair of integers
            Ok(Some((start, end))) => {
                // For each integer in the closed set [start,end], compute the "cycle length"
                // Then return only the maximum "cycle length"
                let max_cycle = (start..=end).map(|n| ThreeN::new(n).count()).max().unwrap();

                // Output is our 2 inputs followed by our maximum cycle length
                println!("{} {} {}", start, end, max_cycle);
            }
            // Handle errors
            Err(err) => match err {
                InputError::IOError(e) => {
                    eprintln!("Failed to read input: {}", e);
                    return Err(InputError::IOError(e));
                }
                InputError::ParseError(e) => eprintln!("Failed to parse input: {}", e),
                InputError::ArgCountError { count: c } => {
                    eprintln!("Expected 2 arguments, got {}", c)
                }
                InputError::ZeroError => eprintln!("0 is not a valid input"),
                // NB: This will never be encountered unless `read_args()` is modified to return it
                InputError::OutOfOrderError => {
                    eprintln!("First argument must be smaller than second argument")
                }
            },
        };
    }

    Ok(())
}
