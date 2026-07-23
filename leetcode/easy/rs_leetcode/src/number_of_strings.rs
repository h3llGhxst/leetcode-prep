struct Solution;

impl Solution {
    pub fn num_f_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut counter = 0; 
        for stri in patterns.iter() { 
            if word.contains(stri) { 
                counter += 1; 
            }
        }
        counter
    }
}
