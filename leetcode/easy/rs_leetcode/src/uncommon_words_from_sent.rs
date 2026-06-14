use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {

        let mut counts : HashMap<String , i32> = HashMap::new(); 

        for word in s1.split(' ').chain(s2.split(' ')) { 
            *counts.entry(word.to_string()).or_insert(0) += 1;
        }

        counts. 
            into_iter(). 
            filter(| (_ ,c)| *c == 1 ). 
            map(|(w,_)| w). 
            collect()
    }
}
