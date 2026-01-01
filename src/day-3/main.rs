use advent_2025_common::{DayPuzzlePart, run_day_puzzle_solver};
use anyhow::{Context, Error};

type BatteryJoltageScalar = u16;

#[derive(Debug)]
struct BatteriesBank {
    joltages: Vec<BatteryJoltageScalar>,
}

impl TryFrom<String> for BatteriesBank {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let joltages = value
            .chars()
            .map(|char| {
                char.to_digit(10)
                    .map(|digit| digit as BatteryJoltageScalar)
                    .with_context(|| format!("BatteriesBank::TryFrom cannot parse char: {char}"))
            })
            .collect::<Result<Vec<BatteryJoltageScalar>, Error>>()?;
        Ok(BatteriesBank { joltages })
    }
}

impl BatteriesBank {
    /// Brute-force search for the largest combined joltage from any two batteries.
    pub fn largest_joltage(&self) -> BatteryJoltageScalar {
        let mut max_joltage = 0;
        let len = self.joltages.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let combined_joltage = self.joltages[i] * 10 + self.joltages[j];
                max_joltage = max_joltage.max(combined_joltage);
            }
        }
        max_joltage
    }
}

/// Computes the total output voltage from all banks by summing their largest joltages.
fn output_voltage(banks: &[BatteriesBank]) -> BatteryJoltageScalar {
    banks.iter().map(|bank| bank.largest_joltage()).sum()
}

fn main() -> Result<(), Error> {
    // Part 1
    run_day_puzzle_solver(3, DayPuzzlePart::One, b'\n', |input: Vec<BatteriesBank>| {
        Ok(output_voltage(&input))
    })?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{BatteriesBank, BatteryJoltageScalar};

    #[test]
    fn test_day_3_bank_largest_joltage() {
        let banks: [(BatteriesBank, BatteryJoltageScalar); 4] = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ]
        .map(|(raw_bank, expected_largest_joltage)| {
            (
                raw_bank.to_string().try_into().unwrap(),
                expected_largest_joltage,
            )
        });
        for (bank, expected_largest_joltage) in banks {
            assert_eq!(bank.largest_joltage(), expected_largest_joltage);
        }
    }
}
