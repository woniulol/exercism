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

        let candidate_charcounter = to_charcounter(*candidate);
        if target_counter.len() == candidate_charcounter.len() {
            let ref mut matched = 0;
            for (k, v) in target_counter.iter() {
                if let Some(counter) = candidate_charcounter.get(k) {
                    if counter == v {
                        *matched += 1;
                        continue;
                    }
                    break;
                }
            }
            if *matched == target_counter.len() {
                result.insert(&possible_anagrams[i]);
            }
        }
    }
    result
}
