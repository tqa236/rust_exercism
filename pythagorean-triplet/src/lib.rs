use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..sum / 3 + 1 {
        for b in (a + 1)..sum / 2 + 1 {
            let c = sum - a - b;

            if b < c && a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }

    triplets
}
