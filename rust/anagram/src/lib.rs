use std::collections::HashSet;
use std::iter::zip;

fn sort_string(s: &str) -> String {
    let mut chars: Vec<char> = s.to_lowercase().chars().collect();
    chars.sort();
    return chars.iter().collect();
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let key_str = sort_string(word);

    let sorted_candidates = possible_anagrams.iter().map(|&s| sort_string(s));

    let mut anagrams = HashSet::new();
    for (sorted, &orig) in zip(sorted_candidates, possible_anagrams) {
        if sorted == key_str && orig.to_lowercase() != word.to_lowercase() {
            println!("{} is an anagram of {}", orig, word);
            anagrams.insert(orig);
        }
    }

    return anagrams;
}
