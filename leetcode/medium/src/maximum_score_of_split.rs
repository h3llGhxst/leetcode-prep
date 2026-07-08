struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        let mut suffix_min = vec![0i64; n];
        suffix_min[n - 1] = nums[n - 1] as i64;

        for i in (0..n - 1).rev() {
            suffix_min[i] = suffix_min[i + 1].min(nums[i] as i64);
        }

        let mut prefix = 0i64;
        let mut ans = i64::MIN;

        for i in 0..n - 1 {
            prefix += nums[i] as i64;
            ans = ans.max(prefix - suffix_min[i + 1]);
        }

        ans
    }
}
