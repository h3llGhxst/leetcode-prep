struct Solution ;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        if n >=  1000 { 
            return n -1000 + 1
        }
        0
    }
}
