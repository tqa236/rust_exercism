pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }

    let mut upper_bound = (n as f64) * (n as f64).ln() * 1.2;
    let mut limit = upper_bound as usize;

    loop {
        let mut sieve = vec![true; limit + 1];
        sieve[0] = false;
        sieve[1] = false;

        for i in 2..=(limit as f64).sqrt() as usize {
            if sieve[i] {
                for j in (i * i..=limit).step_by(i) {
                    sieve[j] = false;
                }
            }
        }

        let primes: Vec<u32> = sieve
            .iter()
            .enumerate()
            .filter_map(|(i, &is_prime)| if is_prime { Some(i as u32) } else { None })
            .collect();

        if primes.len() > n as usize {
            return primes[n as usize];
        }

        limit *= 2;
        upper_bound *= 2.0;
    }
}
