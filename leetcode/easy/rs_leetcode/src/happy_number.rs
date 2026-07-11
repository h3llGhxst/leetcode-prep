struct Solution;

use std::{collections::HashSet};
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {

        let mut seen : HashSet<i32> = HashSet::new();
        
        while n != 1 { 

            if seen.contains(&n){ 
                return false;
            }

            seen.insert(n); 
            n = Solution::square_sum(n); 
        }

        true
    }

    pub fn square_sum(mut n : i32) -> i32 { 
        let mut sum = 0 ;
        let mut digits : Vec<i32> = Vec::new();
        while n > 0 { 
            digits.push(n%10);
            n /= 10;
        }
        sum = digits.iter().map(|val| val*val).sum();
        sum
    }
}
