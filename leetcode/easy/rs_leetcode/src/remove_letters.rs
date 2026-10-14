struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut freqs: HashMap<char, i32> = HashMap::new();

        for c in word.chars() {
            *freqs.entry(c).or_insert(0) += 1;
        }

        for c in word.chars() {
            *freqs.get_mut(&c).unwrap() -= 1;

            let mut counts = freqs.values()
                .filter(|&&v| v > 0)
                .collect::<Vec<_>>();

            counts.sort();

            let valid = counts.first() == counts.last()
                || counts.is_empty();

            *freqs.get_mut(&c).unwrap() += 1;

            if valid {
                return true;
            }
        }

        false
    }
}
