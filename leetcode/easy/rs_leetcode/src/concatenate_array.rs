struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut res :Vec<i32> = Vec::new(); 
        res.extend_from_slice(&nums);
        res.extend_from_slice(&nums);

        res
    }
}
