
struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let n = nums.len() as i32;

        let seen: HashSet<i32> = nums.into_iter().collect();

        let mut x = 1;
        // x > avg  <=>  x * n > sum  -- no division, no floats, no rounding bugs
        while x * n <= sum || seen.contains(&x) {
            x += 1;
        }
        x
    }
}
