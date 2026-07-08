struct Solution;
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut ans: i64 = 1;

        let a = (m - 1) as i64;
        let b = (n - 1) as i64;

        let total = a + b;

        for i in 1..=b {
            ans = ans * (total - b + i) / i;
        }

        ans as i32
    }
}
