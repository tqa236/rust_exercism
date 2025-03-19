#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        dna.chars()
            .enumerate()
            .find(|&(_, c)| !matches!(c, 'A' | 'C' | 'G' | 'T'))
            .map_or_else(|| Ok(Dna(dna.to_string())), |(idx, _)| Err(idx))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        rna.chars()
            .enumerate()
            .find(|&(_, c)| !matches!(c, 'A' | 'C' | 'G' | 'U'))
            .map_or_else(|| Ok(Rna(rna.to_string())), |(idx, _)| Err(idx))
    }
}
