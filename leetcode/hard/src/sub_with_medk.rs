use std::collections::HashMap;
struct Solution; 
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {

        let p = nums.iter().position(|&v| v == k).unwrap();

        let mut left: HashMap<i32, i32> = HashMap::new();
        let mut bal = 0;
        left.insert(0, 1); 
        for i in (0..p).rev() {
            if nums[i] > k { bal += 1; } else { bal -= 1; }
            *left.entry(bal).or_insert(0) += 1;
        }

        let mut ans = 0;
        bal = 0;
        ans += left.get(&0).unwrap_or(&0) + left.get(&1).unwrap_or(&0);
        for i in (p + 1)..nums.len() {
            if nums[i] > k { bal += 1; } else { bal -= 1; }
            ans += left.get(&(-bal)).unwrap_or(&0) + left.get(&(1 - bal)).unwrap_or(&0);
        }

        ans
    }
}
