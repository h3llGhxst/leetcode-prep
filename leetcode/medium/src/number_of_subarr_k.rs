struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut  ans = 0 ; 
        for i in 0..=arr.len() - k as usize{
            let sum = arr[i..i + k as usize].iter().sum::<i32>();

            if sum >= threshold * k as i32 {
                ans += 1;
            }
        }
        ans
    }
}
