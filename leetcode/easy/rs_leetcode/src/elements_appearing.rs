struct Solution;

use std::{collections::HashMap, i32};

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut freq : HashMap<i32, i32> = HashMap::new(); 
        let n = arr.len();

        for &val in arr.iter(){ 
            *freq.entry(val).or_insert(0) += 1;
        }

        for (&val , &freq) in freq.iter(){ 
            if freq >= n as i32 / 4 { 
                return val;
            }
        }
        -1
    }
}
