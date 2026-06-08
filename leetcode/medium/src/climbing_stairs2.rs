struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for j in 1..=n {
            let from1 = dp[j - 1].saturating_add(Self::square(1));
            let from2 = if j >= 2 { dp[j - 2].saturating_add(Self::square(2)) } else { i32::MAX };
            let from3 = if j >= 3 { dp[j - 3].saturating_add(Self::square(3)) } else { i32::MAX };

            dp[j] = costs[j - 1] + Self::compare(from1, from2, from3);
        }

        dp[n]
    }

    fn compare(a: i32, b: i32, c: i32) -> i32 {
        a.min(b.min(c))
    }

    fn square(a: i32) -> i32 {
        a * a
    }
}
