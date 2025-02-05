pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::new();
    let mut prev_char: Option<char> = None;

    for ch in phrase.chars() {
        match ch {
            ' ' | '-' => {
                prev_char = None;
            }
            _ if ch.is_alphabetic() => {
                if prev_char.is_none()
                    || (prev_char.is_some()
                        && prev_char.unwrap().is_lowercase()
                        && ch.is_uppercase())
                {
                    acronym.push(ch.to_ascii_uppercase());
                }
                prev_char = Some(ch);
            }
            _ => continue,
        }
    }

    acronym
}
