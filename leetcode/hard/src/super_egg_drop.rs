
struct Solution;

impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; k + 1]; 
        let mut moves = 0;

        while dp[k] < n {
            moves += 1;

            for j in (1..=k).rev() {
                dp[j] = dp[j - 1] + dp[j] + 1;
            }
        }

        moves
    }
}
