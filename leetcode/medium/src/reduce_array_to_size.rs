
struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        
        let  target = arr.len() as i32 / 2 as i32;
        let mut frequnces = HashMap::new(); 

        for i in 0..arr.len() { 
            *frequnces.entry(arr[i]).or_insert(0) += 1;
        }


        let mut count = 0;

        let mut sorted : Vec<(i32 , i32)> = frequnces.clone().into_iter().collect();
        sorted.sort_by(|a , b| b.1.cmp(&a.1));
        
        let mut sum = 0 ;

        for i in 0..sorted.len() { 
            let val = sorted[i].1 ;

            count += 1; 
            sum += val; 

            if sum >= target { 
                return count;
            }
        }
        count
    }
}
