struct Solution;
impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
       let mut nums = nums; 
       let mut count :  i32 = 0;
       let mut cur_end :i32 = 0; 
       nums.sort_unstable_by(|a , b| a[0].cmp(&b[0]));

       for interval in &nums{ 
           let start = interval[0]; 
           let end = interval[1]; 
           let effective_start = start.max(cur_end +1); 
           count += (end - effective_start +1).max(0);
           cur_end = cur_end.max(end);
       }
       count
    }
}
