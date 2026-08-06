struct Solution;
impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let num : Vec<i32> = num.chars().map(|c| c.to_digit(10).unwrap() as i32 )
            .collect();

        let mut odd_sum = 0 ;
        let mut even_sum= 0 ;


        for ( i , val) in num.iter().enumerate() { 
            if i % 2 == 0 { 
                even_sum += *val
            } else { 
                odd_sum += *val
            }
        }
        odd_sum == even_sum
    }
}
