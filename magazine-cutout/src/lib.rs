// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words: HashMap<&str, i32> = HashMap::new();

    for word in note {
        let counter = words.entry(word).or_insert(0);
        *counter += 1;
    }

    for word in magazine {
        let counter = words.entry(word).or_insert(0);
        *counter -= 1;
    }

    !words.values().any(|x| *x > 0)
}
