use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T' {
        return Err(nucleotide);
    }

    for c in dna.chars() {
        if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
            return Err(c);
        }
    }

    let count = dna.chars().filter(|&c| c == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    counts.insert('A', 0);
    counts.insert('C', 0);
    counts.insert('G', 0);
    counts.insert('T', 0);

    for c in dna.chars() {
        if c != 'A' && c != 'C' && c != 'G' && c != 'T' {
            return Err(c);
        }
        *counts.get_mut(&c).unwrap() += 1;
    }

    Ok(counts)
}
