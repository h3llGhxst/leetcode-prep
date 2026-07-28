
struct  Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {

        let mut sum :i32 = 0; 
        let mut product :i32 = 1; 
        let vals :Vec<i32> = Solution::get_num(n); 

        for val in  vals.iter() { 
            sum += *val;
            product *= *val;
        }


        if n %(product + sum) == 0 {
            return true;
        }
        false

   }

    fn get_num(mut num : i32) -> Vec<i32> {
        let mut nums : Vec<i32> = Vec::new();

        while num > 0 { 
            let d = num % 10 ; 
            nums.push(d);
            num /= 10;
        }
        nums
    }
}
