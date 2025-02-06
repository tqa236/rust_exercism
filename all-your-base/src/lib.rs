#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Validate base inputs
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // Special case for empty input or all zeros
    if number.is_empty() || number.iter().all(|&x| x == 0) {
        return Ok(vec![0]);
    }

    // Validate input digits
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
    }

    // Convert input to decimal (base 10)
    let decimal = number.iter().fold(0, |acc, &digit| acc * from_base + digit);

    // Convert decimal to output base
    if decimal == 0 {
        return Ok(vec![0]);
    }

    let mut output_digits = Vec::new();
    let mut current = decimal;

    while current > 0 {
        output_digits.push(current % to_base);
        current /= to_base;
    }

    // Reverse to get correct digit order
    output_digits.reverse();

    Ok(output_digits)
}
