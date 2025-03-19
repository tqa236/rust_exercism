pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = Vec::new();

    for bottles in (start_bottles - take_down + 1)..=start_bottles {
        let verse = format!(
            "{} green bottle{} hanging on the wall,\n{} green bottle{} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {} green bottle{} hanging on the wall.",
            number_to_word(bottles, true),  
            if bottles == 1 { "" } else { "s" },
            number_to_word(bottles, true),  
            if bottles == 1 { "" } else { "s" },
            number_to_word(bottles - 1, false), 
            if bottles - 1 == 1 { "" } else { "s" }
        );

        result.push(verse);
    }

    result.reverse();
    result.join("\n\n")
}

fn number_to_word(num: u32, capitalize: bool) -> String {
    let word = match num {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => return num.to_string(),
    };

    if capitalize {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
        }
    } else {
        word.to_string()
    }
}
