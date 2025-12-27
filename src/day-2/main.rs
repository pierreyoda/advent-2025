use advent_2025_common::{DayPuzzlePart, run_day_puzzle_solver};
use anyhow::{Context, Error, anyhow};

type ProductIDScalar = u64;

#[derive(Debug)]
pub struct ProductIDsRange {
    first_id: ProductIDScalar,
    last_id: ProductIDScalar,
}

impl TryFrom<String> for ProductIDsRange {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('-').collect();
        if parts.len() != 2 {
            return Err(anyhow!(
                "ProductIDsRange::TryFrom invalid range format: {}",
                value
            ));
        }

        let first_id: ProductIDScalar = parts[0]
            .parse()
            .with_context(|| format!("ProductIDsRange::TryFrom invalid first ID: {}", parts[0]))?;
        let last_id: ProductIDScalar = parts[1]
            .parse()
            .with_context(|| format!("ProductIDsRange::TryFrom invalid last ID: {}", parts[1]))?;

        Ok(ProductIDsRange { first_id, last_id })
    }
}

fn main() -> Result<(), Error> {
    // Part 1
    run_day_puzzle_solver(
        2,
        DayPuzzlePart::One,
        b',',
        |input: Vec<ProductIDsRange>| {
            println!("{}", input.len());
            Ok(())
        },
    )?;

    Ok(())
}
