use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen = HashSet::new();

    for c in candidate.chars() {
        if c == ' ' || c == '-' {
            continue;
        }
        let c = c.to_lowercase().next().unwrap();
        if !seen.insert(c) {
            return false;
        }
    }

    true
}
