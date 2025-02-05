pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }

    let upper_bound = upper_bound as usize;
    let mut is_prime = vec![true; upper_bound + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let limit = (upper_bound as f64).sqrt() as usize;
    for i in 2..=limit {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= upper_bound {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(index, &is_prime)| if is_prime { Some(index as u64) } else { None })
        .collect()
}
