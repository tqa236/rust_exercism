#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let sum: u64 = (1..=num / 2).filter(|&i| num % i == 0).sum();

    Some(match sum {
        s if s == num => Classification::Perfect,
        s if s > num => Classification::Abundant,
        _ => Classification::Deficient,
    })
}
