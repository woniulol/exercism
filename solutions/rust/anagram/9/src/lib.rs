use std::collections::{HashMap, HashSet};

type CharCounter = HashMap<char, usize>;

fn to_charcounter(word: &str) -> CharCounter {
    let mut counter = CharCounter::new();
    for c in word.chars() {
        counter.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    counter
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_charcounter = to_charcounter(&word_lowercase);

    possible_anagrams
        .iter()
        .filter(|item| {
            let candidate_lowercase = item.to_lowercase();
            candidate_lowercase != word_lowercase
                && word_charcounter == to_charcounter(&candidate_lowercase)
        })
        .map(|item| *item)
        .collect()
}
