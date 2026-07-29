struct Solution;

use std::{char, collections::HashSet};
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let set : HashSet<char> = jewels.chars().collect();
        let mut count = 0 ; 
        for c in stones.chars() { 
            if set.contains(&c) { 
                count += 1;
            }
        }

        count
    }
}
