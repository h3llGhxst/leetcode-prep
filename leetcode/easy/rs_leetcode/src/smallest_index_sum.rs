struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
    let mut res = 0 ;
    let mut sum = 0 ;
    for (i,val) in nums.iter().enumerate() { 

        sum = 0 ;
        let mut v = *val;

        while v != 0 {
            sum += v % 10 ; 
            v = v / 10;
        }

        if sum  == i as i32 { 
            return i as i32
        }
    }
    -1
    }
}
