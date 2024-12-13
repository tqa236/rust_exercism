use std::collections::HashSet;



pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
 
    fn sorted_lowercase(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    }

    let word_lowercase = word.to_lowercase();
    let sorted_word = sorted_lowercase(word);

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
            let candidate_lowercase = candidate.to_lowercase();
            candidate_lowercase != word_lowercase
                && sorted_lowercase(candidate) == sorted_word 
        })
        .copied()
        .collect()
}
