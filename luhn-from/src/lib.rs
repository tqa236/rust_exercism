pub struct Luhn {
    number: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let sanitized = self.number.replace(char::is_whitespace, "");
        if sanitized.len() <= 1 || !sanitized.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        let mut sum = 0;
        let mut alternate = false;

        for ch in sanitized.chars().rev() {
            let mut n = ch.to_digit(10).unwrap() as usize;
            if alternate {
                n *= 2;
                if n > 9 {
                    n -= 9;
                }
            }
            sum += n;
            alternate = !alternate;
        }

        sum % 10 == 0
    }
}

impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn {
            number: input.to_string(),
        }
    }
}
