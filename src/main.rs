use threen::{read_args, InputError, ThreeN};

fn main() -> Result<(), InputError> {
    loop {
        match read_args() {
            Ok(None) => break,
            Ok(Some((start, end))) => {
                let max_cycle = (start..=end).map(|n| ThreeN::new(n).count()).max().unwrap();

                println!("{} {} {}", start, end, max_cycle);
            }
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
                InputError::OutOfOrderError => {
                    eprintln!("First argument must be smaller than second argument")
                }
            },
        };
    }

    Ok(())
}
