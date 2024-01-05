use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut occurrences: HashMap<String, u32> = HashMap::new();

    words
        .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
        .for_each(|word| {
            let processed_word = word
                .trim_start_matches(|ch: char| ch.is_ascii_punctuation())
                .trim_end_matches(|ch: char| ch.is_ascii_punctuation())
                .to_lowercase();

            if !processed_word.is_empty() {
                *occurrences.entry(processed_word).or_default() += 1;
            }
        });

    occurrences
}
