struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut freq : HashMap<i32 , i32> = HashMap::new(); 
        let mut res : Vec<i32> = Vec::new();

        let s1 : HashSet<_> = nums1.iter().collect();
        let s2 : HashSet<_> = nums2.iter().collect();
        let s3 : HashSet<_> = nums3.iter().collect();

        for val in s1.iter().chain(s2.iter()).chain(s3.iter()) { 
            *freq.entry(**val).or_insert(0) += 1;
        }

        for (&key , &freq) in freq.iter() { 
            if freq >= 2  { 
                res.push(key);
            }
        }
        res
    }
}
