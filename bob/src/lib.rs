pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();

    if trimmed_message.is_empty() {
        return "Fine. Be that way!";
    }

    let is_shouting = trimmed_message == trimmed_message.to_uppercase()
        && trimmed_message.chars().any(|c| c.is_alphabetic());

    let is_question = trimmed_message.ends_with('?');

    if is_shouting && is_question {
        return "Calm down, I know what I'm doing!";
    }

    if is_shouting {
        return "Whoa, chill out!";
    }

    if is_question {
        return "Sure.";
    }

    "Whatever."
}
