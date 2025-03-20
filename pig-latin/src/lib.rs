pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut chars = word.chars();

    if let Some(first_char) = chars.next() {
        if vowels.contains(&first_char)
            || (first_char == 'y' && chars.clone().next() == Some('t'))
            || (first_char == 'x' && chars.clone().next() == Some('r'))
        {
            // Rule 1: Starts with a vowel or "yt" or "xr"
            return format!("{}ay", word);
        } else {
            // Rule 2, 3, and 4: Starts with consonants
            let mut consonants = vec![first_char];
            while let Some(c) = chars.clone().next() {
                if (consonants.last() == Some(&'q') && c == 'u')
                    || (!vowels.contains(&c) && c != 'y')
                {
                    consonants.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let remaining: String = chars.collect();
            return format!(
                "{}{}ay",
                remaining,
                consonants.into_iter().collect::<String>()
            );
        }
    }
    word.to_string()
}
