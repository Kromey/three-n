use std::io;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum InputError {
    IOError(io::Error),
    ParseError(ParseIntError),
    ArgCountError { count: usize },
    ZeroError,
    OutOfOrderError,
}

impl From<ParseIntError> for InputError {
    fn from(e: ParseIntError) -> InputError {
        InputError::ParseError(e)
    }
}

impl From<io::Error> for InputError {
    fn from(e: io::Error) -> InputError {
        InputError::IOError(e)
    }
}
