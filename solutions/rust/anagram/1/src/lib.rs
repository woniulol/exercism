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
    let mut result: HashSet<&'a str> = HashSet::new();
    let target_counter = to_charcounter(word);
    let possible_counters = possible_anagrams
        .into_iter()
        .map(|item| to_charcounter(item));

    for (i, possible_counter) in possible_counters.enumerate() {
        if target_counter.len() == possible_counter.len() {
            for (k, v) in target_counter.iter() {
                if let Some(counter) = possible_counter.get(k) {
                    if counter == v {
                        continue;
                    }
                } else {
                    break;
                }
                result.insert(&possible_anagrams[i]);
            }
        }
        continue;
    }
    result
}
