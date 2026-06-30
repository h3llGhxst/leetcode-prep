struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n <= 1 { 
            return n;
        }
        Solution::fib(n-1) + Solution::fib(n-2)
    }
}
