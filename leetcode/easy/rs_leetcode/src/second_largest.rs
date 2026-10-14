

struct Solution;

impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut vals: Vec<i32> = Vec::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                vals.push(c.to_digit(10).unwrap() as i32);
            }
        }

        vals.sort_unstable();
        vals.dedup();

        if vals.len() < 2 {
            return -1;
        }

        vals[vals.len() - 2]
    }
}
