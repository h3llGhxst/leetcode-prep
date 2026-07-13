struct Solution;

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words :  Vec<&str> = text.split(' ').collect();
        let mut thirds : Vec<String> = Vec::new();

        for i in 0..words.len() -2  { 
            if words[i] == first && words[i+1] == second { 
                thirds.push(words[i+2].to_string());
            }
        }

        thirds
    }
}
