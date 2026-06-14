struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let bytes = s.as_bytes(); 
        let n = bytes.len(); 
        let mut results = Vec::new(); 
        let mut start = 0; 


        for i in 0..n { 

            if i == n -1 || bytes[i] != bytes[i+1] { 
                if i - start + 1 >= 3 { 
                    results.push(vec![start as i32 , i as i32]);
                }

                start = i + 1;
            }
        }
        results
    }
}
