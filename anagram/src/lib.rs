use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = sort(&word_lower);

    possible_anagrams
        .iter()
        .filter(|sus| {
            let sus_lower = sus.to_lowercase();
            word_lower != sus_lower && word_sorted == sort(&sus_lower)
        })
        .copied()
        .collect() // collect() will automatically infer the correct collection type based off the function return type
}

fn sort(word: &str) -> Vec<&str> {
    let mut word_sorted = word.graphemes(true).collect::<Vec<&str>>();
    word_sorted.sort_unstable();
    word_sorted
}
