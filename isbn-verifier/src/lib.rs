pub fn is_valid_isbn(isbn: &str) -> bool {
    let cleaned = isbn.replace('-', "");

    if cleaned.len() != 10 {
        return false;
    }

    let mut checksum = 0;
    for (i, c) in cleaned.chars().enumerate() {
        let digit = match (i, c) {
            (9, 'X') => 10,
            (9, d) => match d.to_digit(10) {
                Some(digit) => digit,
                None => return false,
            },
            (_, d) => match d.to_digit(10) {
                Some(digit) => digit,
                None => return false,
            },
        };

        checksum += digit * (10 - i as u32);
    }

    checksum % 11 == 0
}
