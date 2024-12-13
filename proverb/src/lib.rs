pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut proverb = Vec::new();

    for window in list.windows(2) {
        proverb.push(format!(
            "For want of a {} the {} was lost.",
            window[0], window[1]
        ));
    }

    if let Some(first) = list.first() {
        proverb.push(format!("And all for the want of a {}.", first));
    }

    proverb.join("\n")
}
