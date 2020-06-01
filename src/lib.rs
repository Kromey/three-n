mod error;

pub use self::error::InputError;
use std::io;

/// Read arguments from stdin
///
/// This function attempts to read a pair of inputs on a single line of input from stdin. If
/// successful they are returned as a tuple; we return `None` if the input line is blank.
///
/// The inputs are expected to be positive (non-zero) integers. Any errors are represented in the
/// `InputError` variant returned.
pub fn read_args() -> Result<Option<(u32, u32)>, InputError> {
    // Create a String buffer
    let mut input = String::new();
    // Read a line of input from stdin
    io::stdin().read_line(&mut input)?;

    let args: Vec<u32> = input
        // Trim leading/trailing whitespace
        .trim()
        // Split on any amount of whitespace
        .split_whitespace()
        .filter_map(|arg| {
            if arg.is_empty() {
                // Filter out empty slices
                None
            } else {
                // Parse into an unsigned integer
                Some(arg.parse::<u32>())
            }
        })
        // Any parse errors are returned to calling code here
        .collect::<Result<_, _>>()?;

    if args.is_empty() {
        // Empty line of input, which we represent as `None`
        Ok(None)
    } else if args.len() != 2 {
        // We expected 2 arguments but got come other number
        Err(InputError::ArgCountError { count: args.len() })
    } else if args[0] == 0 || args[1] == 0 {
        // Input cannot be zero
        Err(InputError::ZeroError)
    } else if args[0] > args[1] {
        // We treat out-of-order inputs (second smaller than first) as a warning
        eprintln!("WARNING: Input arguments are out of order.");

        // Just flip them around and the program will work
        Ok(Some((args[1], args[0])))
    } else {
        // A pair of correctly-entered inputs, imagine that!
        Ok(Some((args[0], args[1])))
    }
}

/// An iterator for the 3n+1 algorithm
///
/// This struct is an iterator that, starting from a positive (non-zero) integer, will compute each
/// step of the 3n+1 algorithm until terminating after the final 1 result.
///
/// The complete algorithm is:
///  * If the input is 1, terminate
///  * If the input is odd, return `3 * num + 1`
///  * If the input is even, return `num / 2`
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use threen::ThreeN;
///
/// let cycle: Vec<u32> = ThreeN::new(10).collect();
///
/// assert_eq!(cycle, vec![10, 5, 16, 8, 4, 2, 1]);
/// ```
///
/// Counting cycle length:
///
/// ```
/// use threen::ThreeN;
///
/// let cycle_len = ThreeN::new(10).count();
///
/// assert_eq!(cycle_len, 7);
/// ```
pub struct ThreeN {
    next: Option<u32>,
}

impl ThreeN {
    /// Create a new ThreeN iterator starting at `n`
    ///
    /// # Panics
    ///
    /// This will panic if `n` is 0.
    pub fn new(n: u32) -> ThreeN {
        assert!(n > 0);

        ThreeN { next: Some(n) }
    }

    /// Compute the 3n+1 algorithm result for `num`
    fn threen(num: u32) -> Option<u32> {
        match num {
            1 => None,
            n if n % 2 == 1 => Some(3 * n + 1),
            n => Some(n / 2),
        }
    }
}

impl Iterator for ThreeN {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // Our previous `next` value is now our current value, hold onto it for later
        let current = self.next;

        // If our previous result was a None, we're done iterating
        if let Some(num) = self.next {
            // Compute the next value of the algorithm
            self.next = ThreeN::threen(num);
        }

        // Return our current value, which was our previous `next` value
        current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_terminates() {
        assert_eq!(ThreeN::threen(1), None);
    }

    #[test]
    fn odd_is_3n_plus_one() {
        assert_eq!(ThreeN::threen(3), Some(10));
        assert_eq!(ThreeN::threen(5), Some(16));
        assert_eq!(ThreeN::threen(7), Some(22));
    }

    #[test]
    fn even_is_half() {
        assert_eq!(ThreeN::threen(2), Some(1));
        assert_eq!(ThreeN::threen(4), Some(2));
        assert_eq!(ThreeN::threen(6), Some(3));
    }

    #[test]
    fn cycle() {
        assert_eq!(ThreeN::new(1).collect::<Vec<u32>>(), vec![1]);
        assert_eq!(ThreeN::new(2).collect::<Vec<u32>>(), vec![2, 1]);
        assert_eq!(
            ThreeN::new(3).collect::<Vec<u32>>(),
            vec![3, 10, 5, 16, 8, 4, 2, 1]
        );
    }
}
