pub mod error;

pub use self::error::InputError;
use std::io;

pub fn read_args() -> Result<Option<(u32, u32)>, InputError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let args: Vec<u32> = input
        .trim()
        .split_whitespace()
        .filter_map(|arg| {
            if arg.is_empty() {
                None
            } else {
                Some(arg.parse::<u32>())
            }
        })
        .collect::<Result<_, _>>()?;

    if args.is_empty() {
        Ok(None)
    } else if args.len() != 2 {
        Err(InputError::ArgCountError { count: args.len() })
    } else if args[0] == 0 || args[1] == 0 {
        Err(InputError::ZeroError)
    } else if args[0] > args[1] {
        eprintln!("WARNING: Input arguments are out of order.");

        Ok(Some((args[1], args[0])))
    } else {
        Ok(Some((args[0], args[1])))
    }
}

pub struct ThreeN {
    next: Option<u32>,
}

impl ThreeN {
    pub fn new(n: u32) -> ThreeN {
        assert!(n > 0);

        ThreeN { next: Some(n) }
    }

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
        let current = self.next;

        if let Some(num) = self.next {
            self.next = ThreeN::threen(num);
        }

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
