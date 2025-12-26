use anyhow::{Context, Error, Result, anyhow};

use advent_2025_common::{DayPuzzlePart, run_day_puzzle_solver};

type VaultDialScalar = u16;
const VAULT_DIAL_MAX_VALUE: VaultDialScalar = 99;
const VAULT_DIAL_ROTATION_RANGE: VaultDialScalar = VAULT_DIAL_MAX_VALUE + 1;
const VAULT_DIAL_INITIAL_VALUE: VaultDialScalar = 50;
const VAULT_DIAL_DIRECTION_LEFT_CHAR: char = 'L';
const VAULT_DIAL_DIRECTION_RIGHT_CHAR: char = 'R';

#[derive(Debug, PartialEq, Eq)]
enum VaultDialRotationDirection {
    /// Toward lower numbers.
    Left,
    /// Toward higher numbers.
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VaultDialRotation {
    direction: VaultDialRotationDirection,
    distance: VaultDialScalar,
}

impl TryFrom<String> for VaultDialRotation {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let direction_char = value.chars().next().with_context(|| {
            format!("DialRotation::TryFrom invalid first character in value: {value}")
        })?;
        let distance_raw = &value[1..];
        let distance: VaultDialScalar = distance_raw.parse().with_context(|| {
            format!(
                "DialRotation::TryFrom invalid distance scalar: {}",
                distance_raw
            )
        })?;

        let direction = match direction_char {
            VAULT_DIAL_DIRECTION_LEFT_CHAR => VaultDialRotationDirection::Left,
            VAULT_DIAL_DIRECTION_RIGHT_CHAR => VaultDialRotationDirection::Right,
            _ => {
                return Err(anyhow!(
                    "DialRotation::TryFrom invalid dial rotation direction cha"
                ));
            }
        };

        Ok(VaultDialRotation {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
pub struct VaultDial {
    value: VaultDialScalar,
}

impl VaultDial {
    pub fn new(initial_value: Option<VaultDialScalar>) -> Self {
        VaultDial {
            value: initial_value.unwrap_or(VAULT_DIAL_INITIAL_VALUE),
        }
    }

    pub fn reset(&mut self, to_value: Option<VaultDialScalar>) {
        self.value = to_value.unwrap_or(VAULT_DIAL_INITIAL_VALUE);
    }

    pub fn get_value(&self) -> VaultDialScalar {
        self.value
    }

    pub fn solve(&mut self, rotations: &[VaultDialRotation], part: &DayPuzzlePart) -> i32 {
        let mut dial_zero_values = 0;
        for dial_rotation in rotations {
            match part {
                DayPuzzlePart::One => {
                    self.rotate(dial_rotation);
                    if self.value == 0 {
                        dial_zero_values += 1;
                    }
                }
                DayPuzzlePart::Two => dial_zero_values += self.rotate_part_2(dial_rotation),
            }
        }
        dial_zero_values
    }

    /// Rotates the vault dial according to the given rotation.
    pub fn rotate(&mut self, rotation: &VaultDialRotation) {
        match rotation.direction {
            VaultDialRotationDirection::Left => {
                self.value = (self.value + VAULT_DIAL_ROTATION_RANGE
                    - (rotation.distance % VAULT_DIAL_ROTATION_RANGE))
                    % VAULT_DIAL_ROTATION_RANGE;
            }
            VaultDialRotationDirection::Right => {
                self.value = (self.value + rotation.distance) % (VAULT_DIAL_MAX_VALUE + 1);
            }
        }
    }

    /// Rotates the vault dial according to the given rotation.
    ///
    /// Returns the number of times the dial pointed at 0 during the rotation.
    pub fn rotate_part_2(&mut self, rotation: &VaultDialRotation) -> i32 {
        let mut zero_value_passes = 0;
        let mut incrementally_rotate = |direction: &VaultDialRotationDirection| {
            self.value = match direction {
                VaultDialRotationDirection::Left => {
                    if self.value == 0 {
                        VAULT_DIAL_MAX_VALUE
                    } else {
                        self.value - 1
                    }
                }
                VaultDialRotationDirection::Right => {
                    if self.value == VAULT_DIAL_MAX_VALUE {
                        0
                    } else {
                        self.value + 1
                    }
                }
            };
            if self.value == 0 {
                zero_value_passes += 1;
            }
        };

        for _ in 0..rotation.distance {
            incrementally_rotate(&rotation.direction);
        }
        zero_value_passes
    }
}

fn main() -> Result<()> {
    // Part 1
    run_day_puzzle_solver(
        1,
        DayPuzzlePart::One,
        b'\n',
        |input: Vec<VaultDialRotation>| {
            let mut dial = VaultDial::new(None);
            Ok(dial.solve(&input, &DayPuzzlePart::One))
        },
    )?;

    // Part 2
    run_day_puzzle_solver(
        1,
        DayPuzzlePart::Two,
        b'\n',
        |input: Vec<VaultDialRotation>| {
            let mut dial = VaultDial::new(None);
            Ok(dial.solve(&input, &DayPuzzlePart::Two))
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use advent_2025_common::DayPuzzlePart;

    use crate::{VaultDial, VaultDialRotation, VaultDialRotationDirection};

    fn get_testing_rotations() -> Vec<VaultDialRotation> {
        vec![
            VaultDialRotation::try_from("L68".to_string()).unwrap(),
            VaultDialRotation::try_from("L30".to_string()).unwrap(),
            VaultDialRotation::try_from("R48".to_string()).unwrap(),
            VaultDialRotation::try_from("L5".to_string()).unwrap(),
            VaultDialRotation::try_from("R60".to_string()).unwrap(),
            VaultDialRotation::try_from("L55".to_string()).unwrap(),
            VaultDialRotation::try_from("L1".to_string()).unwrap(),
            VaultDialRotation::try_from("L99".to_string()).unwrap(),
            VaultDialRotation::try_from("R14".to_string()).unwrap(),
            VaultDialRotation::try_from("L82".to_string()).unwrap(),
        ]
    }

    #[test]
    fn test_day_1_vault_dial_rotation() {
        let mut vault_dial = VaultDial::new(Some(5));

        let rotation1 = VaultDialRotation::try_from("L10".to_string()).unwrap();
        assert_eq!(
            rotation1,
            VaultDialRotation {
                direction: VaultDialRotationDirection::Left,
                distance: 10,
            }
        );
        vault_dial.rotate(&rotation1);
        assert_eq!(vault_dial.get_value(), 95);

        let rotation2 = VaultDialRotation::try_from("R5".to_string()).unwrap();
        assert_eq!(
            rotation2,
            VaultDialRotation {
                direction: VaultDialRotationDirection::Right,
                distance: 5,
            }
        );
        vault_dial.rotate(&rotation2);
        assert_eq!(vault_dial.get_value(), 0);
    }

    #[test]
    fn test_day_1_vault_dial_solving() {
        let rotations = get_testing_rotations();
        let mut vault_dial = VaultDial::new(None);
        assert_eq!(vault_dial.solve(&rotations, &DayPuzzlePart::One), 3);
    }

    #[test]
    fn test_day_1_part_2_vault_dial_solving() {
        let rotations = get_testing_rotations();
        let mut vault_dial = VaultDial::new(None);
        assert_eq!(vault_dial.solve(&rotations, &DayPuzzlePart::Two), 6);
    }
}
