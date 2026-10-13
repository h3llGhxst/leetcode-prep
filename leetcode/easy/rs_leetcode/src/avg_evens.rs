struct Solution;

impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let mut sum : i32 = 0;
        let mut counter = 0 ; 
        for val in nums { 
            if (val % 2 == 0 ) && val % 3 == 0 { 
                sum += val;
                counter += 1; 
            }
        }
        if counter == 0 { 
            return  0;
        }
        sum /  counter
    }
}
