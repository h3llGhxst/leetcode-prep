struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len(); 
        let mut jumps = 0;
        let mut curr_end = 0 ; 
        let mut furthest = 0 ; 

        for i in 0..n-1 { 
            furthest =furthest.max(i + nums[i] as usize);

            if i == curr_end {
                jumps += 1; 
                curr_end = furthest;
            }
        }

        jumps as i32
    }

}
