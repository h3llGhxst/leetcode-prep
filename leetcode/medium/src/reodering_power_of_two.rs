struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let target = Self::sorted_digits(n); 

        let mut power  : i64 = 1; 


        while power <= 1_000_000_000 {
            if Self::sorted_digits(power as i32) == target { 
                return true;
            }
            power *= 2; 
        }
        false 
    }

    fn sorted_digits(mut val: i32 ) -> Vec<i32> {
        let mut sorted_digits : Vec<i32> = Vec::new();
        while val > 0 { 
            sorted_digits.push(val % 10);
            val  /= 10 ;
        }
        sorted_digits.sort_unstable();
        sorted_digits
    }
}
