use std::env;
use threen::{ThreeN, InputError};

fn main() -> Result<(), InputError> {
    let mut args: Vec<u32> = env::args()
        .skip(1)
        .map(|arg| arg.parse::<u32>())
        .collect::<Result<_, _>>()?;

    if args.len() != 2 {
        return Err(InputError::ArgCountError{ count: args.len() });
    } else if args[0] == 0 || args[1] == 0 {
        return Err(InputError::ZeroError);
    } else if args[0] > args[1] {
        eprintln!("WARNING: Input arguments are out of order.");
        let arg = args[0];
        args[0] = args[1];
        args[1] = arg;
    }

    let max_cycle = (args[0]..=args[1]).map(|n| ThreeN::new(n).count()).max().unwrap();

    println!("{} {} {}", args[0], args[1], max_cycle);

    Ok(())
}
