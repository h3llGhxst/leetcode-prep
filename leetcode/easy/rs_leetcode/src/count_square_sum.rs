
struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut count = 0;
        for a in 1..=n {
            for b in 1..=n {
                let sum = a * a + b * b;
                let c = (sum as f64).sqrt() as i32;
                if c <= n && c * c == sum {
                    count += 1;
                }
            }
        }
        count
    }
}
