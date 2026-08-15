use std::collections::{HashMap, HashSet};

type CharCounter = HashMap<char, usize>;

fn to_charcounter(word: &str) -> CharCounter {
    let mut counter = CharCounter::new();
    for c in word.to_lowercase().chars() {
        counter.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    println!("{}: {:?}", word, counter);
    counter
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let target_counter = to_charcounter(word);

    for (i, candidate) in possible_anagrams.iter().enumerate() {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        if target_counter == to_charcounter(*candidate) {
            result.insert(&possible_anagrams[i]);
        }
    }
    result
}
