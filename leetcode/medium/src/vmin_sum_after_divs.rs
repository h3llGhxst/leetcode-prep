
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn min_array_sum(nums: Vec<i32>, k: i32) -> i64 {

        let k = k as i64;
        let total: i64 = nums.iter().map(|&x| x as i64).sum();

        let mut best: HashMap<i64, i64> = HashMap::new();
        best.insert(0, 0); // empty prefix (j=0): dp[0] - prefix[0] = 0

        let quorlathin = nums.clone();

        let mut prefix: i64 = 0;
        let mut dp_prev: i64 = 0;

        for &val in quorlathin.iter() {
            prefix += val as i64;
            let r = ((prefix % k) + k) % k; // normalize in case of negatives

            let candidate = match best.get(&r) {
                Some(&b) => prefix + b,
                None => i64::MIN,
            };
            let dp_i = dp_prev.max(candidate);

            let entry = best.entry(r).or_insert(i64::MIN);
            *entry = (*entry).max(dp_i - prefix);

            dp_prev = dp_i;
        }
        total - dp_prev
    }
}
