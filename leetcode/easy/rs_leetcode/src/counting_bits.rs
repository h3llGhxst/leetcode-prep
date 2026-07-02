struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res : Vec<i32> = Vec::new();
        for i in 0..=n { 
            res.push(i.count_ones() as i32);
        }
        res
    }
}
