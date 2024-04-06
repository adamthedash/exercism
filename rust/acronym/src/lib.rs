pub fn abbreviate(phrase: &str) -> String {
    let phrase_chars = phrase.chars().collect::<Vec<char>>();

    let mut cleaned = String::new();
    for i in 0..phrase_chars.len() {
        let c = phrase_chars[i];

        if c == '-' || c == ' ' {
            cleaned.push(' ');
            continue;
        }

        if !c.is_alphabetic() { continue; }

        if c.is_uppercase() && i > 0 && phrase_chars[i - 1].is_lowercase() {
            cleaned.push(' ');
        }

        cleaned.push(c);
    }

    let processed = cleaned
        .split_whitespace()
        .filter(|&c| c.len() > 0)
        .map(|s| s.chars().next().unwrap())
        .map(|c| c.to_uppercase().to_string())
        .collect::<Vec<String>>();

    processed.join("")
}
