use std::io;
use std::num::ParseIntError;

/// The error type for reading input arguments.
#[derive(Debug)]
pub enum InputError {
    /// Encountered an error reading from stdin
    IOError(io::Error),
    /// Failed to parse arguments as unsigned integers
    ParseError(ParseIntError),
    /// Too few/too many arguments supplied
    ArgCountError { count: usize },
    /// Zero supplied as an argument
    ZeroError,
    /// Arguments were supplied out of order (second argument larger than first)
    OutOfOrderError,
}

/// Convert from ParseIntError by wrapping it in our ParseError variant
impl From<ParseIntError> for InputError {
    fn from(e: ParseIntError) -> InputError {
        InputError::ParseError(e)
    }
}

/// Convert from io::Error by wrapping it in out IOError variant
impl From<io::Error> for InputError {
    fn from(e: io::Error) -> InputError {
        InputError::IOError(e)
    }
}
