use std::collections::HashMap;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let codon_table: HashMap<&str, &str> = [
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", "STOP"),
        ("UAG", "STOP"),
        ("UGA", "STOP"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut proteins = Vec::new();
    let mut i = 0;

    while i < rna.len() {
        if i + 2 >= rna.len() {
            return None;
        }
        let codon = &rna[i..i + 3];
        match codon_table.get(codon) {
            Some(&"STOP") => break, // Stop translation if a stop codon is encountered
            Some(&amino_acid) => proteins.push(amino_acid),
            None => return None, // Return None if the codon is unknown
        }
        i += 3;
    }

    Some(proteins)
}
