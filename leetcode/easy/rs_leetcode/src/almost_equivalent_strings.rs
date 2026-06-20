struct Solution;

use std::{char, collections::HashMap};

impl Solution {

    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut freq1: HashMap<char, i32> = HashMap::new();
    let mut freq2: HashMap<char, i32> = HashMap::new();

    for c in word1.chars() {
        *freq1.entry(c).or_insert(0) += 1;
    }
    for c in word2.chars() {
        *freq2.entry(c).or_insert(0) += 1;
    }

    // every char that shows up in EITHER word
        for c in word1.chars().chain(word2.chars()) {
           let a = freq1.get(&c).copied().unwrap_or(0);
            let b = freq2.get(&c).copied().unwrap_or(0);
            if (a - b).abs() > 3 {
                return false;
            }
        }
        true
    }

}
