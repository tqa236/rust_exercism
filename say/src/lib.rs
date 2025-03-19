pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let scales = [
        (1_000_000_000_000_000_000, "quintillion"),
        (1_000_000_000_000_000, "quadrillion"),
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1_000, "thousand"),
        (100, "hundred"),
    ];

    let ones = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let teens = [
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = Vec::new();

    for (divisor, word) in scales.iter() {
        if n >= *divisor {
            let scale_value = n / divisor;
            let sub_encode = encode(scale_value);
            result.push(format!("{} {}", sub_encode, word));
            n %= divisor;
        }
    }

    if n > 0 {
        match n {
            1..=9 => result.push(ones[n as usize].to_string()),
            10..=19 => result.push(teens[(n - 10) as usize].to_string()),
            20..=99 => {
                let ten_digit = n / 10;
                let one_digit = n % 10;
                if one_digit == 0 {
                    result.push(tens[ten_digit as usize].to_string());
                } else {
                    result.push(format!(
                        "{}-{}",
                        tens[ten_digit as usize], ones[one_digit as usize]
                    ));
                }
            }
            _ => result.push(encode(n)),
        }
    }

    result.join(" ")
}
