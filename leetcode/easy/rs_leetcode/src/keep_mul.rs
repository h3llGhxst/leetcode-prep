struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut  copy = original;
        while nums.contains(&copy)  {
            copy = 2 * copy;
        }
    copy
    }
}
