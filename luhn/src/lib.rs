/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");

    if code.chars().any(|c| !c.is_ascii_digit()) {
        return false;
    }

    if code.len() <= 1 {
        return false;
    }

    let mut sum = 0;
    let mut double = false;

    for c in code.chars().rev() {
        let digit = c.to_digit(10).unwrap();

        if double {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }

        double = !double;
    }

    sum % 10 == 0
}
