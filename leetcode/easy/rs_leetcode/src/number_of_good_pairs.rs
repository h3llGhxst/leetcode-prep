
struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut count :i32 = 0; 
        for i in 0..nums.len() { 
            let  prev = nums[i];
            for j in i+1..nums.len() { 
                let next = nums[j];
                if next == prev { 
                    count +=1; 
                }
            }
        }

        count
    }
}
