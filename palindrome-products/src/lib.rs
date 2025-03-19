use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut smallest_palindrome: Option<Palindrome> = None;
    let mut largest_palindrome: Option<Palindrome> = None;

    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                match smallest_palindrome {
                    Some(ref mut sp) if product < sp.value => {
                        sp.value = product;
                        sp.factors = HashSet::from([(i, j)]);
                    }
                    Some(ref mut sp) if product == sp.value => {
                        sp.factors.insert((i, j));
                    }
                    None => {
                        smallest_palindrome = Some(Palindrome {
                            value: product,
                            factors: HashSet::from([(i, j)]),
                        });
                    }
                    _ => {}
                }

                match largest_palindrome {
                    Some(ref mut lp) if product > lp.value => {
                        lp.value = product;
                        lp.factors = HashSet::from([(i, j)]);
                    }
                    Some(ref mut lp) if product == lp.value => {
                        lp.factors.insert((i, j));
                    }
                    None => {
                        largest_palindrome = Some(Palindrome {
                            value: product,
                            factors: HashSet::from([(i, j)]),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    match (smallest_palindrome, largest_palindrome) {
        (Some(smallest), Some(largest)) => Some((smallest, largest)),
        _ => None,
    }
}

fn is_palindrome(num: u64) -> bool {
    let s = num.to_string();
    s == s.chars().rev().collect::<String>()
}
