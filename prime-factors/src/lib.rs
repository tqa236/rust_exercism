pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::new();

    while n.is_multiple_of(2) {
        factors.push(2);
        n /= 2;
    }

    let mut i = 3;
    while i * i <= n {
        while n.is_multiple_of(i) {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}
