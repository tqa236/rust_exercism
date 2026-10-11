pub fn raindrops(n: u32) -> String {
    let sounds = vec![(3, "Pling"), (5, "Plang"), (7, "Plong")];

    let mut result = String::new();
    for (key, value) in &sounds {
        if n.is_multiple_of(*key) {
            result.push_str(value)
        }
    }

    if result.is_empty() {
        result.push_str(&n.to_string())
    }

    result
}
