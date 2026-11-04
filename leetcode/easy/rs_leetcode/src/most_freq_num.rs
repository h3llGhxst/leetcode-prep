struct Solution;


use std::collections::HashMap;

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut res: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;
        let mut ans = -1;

        for pair in nums.windows(2) {
            if pair[0] != key {
                continue;
            }

            let target = pair[1];
            let count = res.entry(target).or_insert(0);
            *count += 1;

            if *count > max_count {
                max_count = *count;
                ans = target;
            }
        }

        ans
    }
}
