use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut sorted_word: Vec<char> = word_lower.chars().collect();
    sorted_word.sort_unstable();

    possible_anagrams.iter()
        .filter(|&&candidate| {
            let candidate_lower = candidate.to_lowercase();
            if candidate_lower == word_lower {
                return false; // Exclude the original word
            }
            let mut sorted_candidate: Vec<char> = candidate_lower.chars().collect();
            sorted_candidate.sort_unstable();
            sorted_candidate == sorted_word
        })
        .cloned()
        .collect()
}