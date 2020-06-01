use threen::{InputError, ThreeN, read_args};

fn main() -> Result<(), InputError> {
    while let Some((start, end)) = read_args()? {
        let max_cycle = (start..=end)
            .map(|n| ThreeN::new(n).count())
            .max()
            .unwrap();

        println!("{} {} {}", start, end, max_cycle);
    }

    Ok(())
}
