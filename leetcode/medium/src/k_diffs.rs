struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for &n in nums.iter() {
            *count.entry(n).or_insert(0) += 1;
        }

        let mut pairs = 0;
        for (&val, &freq) in count.iter() {
            if k == 0 {
                if freq > 1 {
                    pairs += 1;
                }
            } else if count.contains_key(&(val + k)) {
                pairs += 1;
            }
        }

        pairs
    }
}
