use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        seen.insert(0, 1);

        let mut prefix = 0;
        let mut count = 0;

        for x in nums {
            prefix += x;
            if let Some(&c) = seen.get(&(prefix - k)) {
                count += c;
            }
            *seen.entry(prefix).or_insert(0) += 1;
        }

        count
    }
}
