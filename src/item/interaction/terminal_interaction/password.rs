use crate::item::interaction::terminal_interaction::digit::Digit;

pub const PASSWORD_LENGTH: usize = 6;

#[derive(Clone, Default, PartialEq)]
pub struct Password {
    pub digits: [Digit; PASSWORD_LENGTH],
}

impl Password {
    pub fn get_correct() -> Self {
        Self {
            digits: [
                Digit::One,
                Digit::One,
                Digit::One,
                Digit::One,
                Digit::One,
                Digit::One,
            ],
        }
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut digit_string = String::new();

        for digit in &self.digits {
            digit_string.push_str(&digit.to_string())
        }

        write!(f, "{}", digit_string)
    }
}

impl std::str::FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != PASSWORD_LENGTH {
            return Err(());
        }

        let mut six_digits = Password::default();

        for (index, digit_char) in s.chars().enumerate() {
            match Digit::try_from(digit_char) {
                Ok(digit) => six_digits.digits[index] = digit,
                Err(_error) => return Err(()),
            };
        }

        Ok(six_digits)
    }
}
