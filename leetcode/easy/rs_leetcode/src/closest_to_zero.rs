struct  Solution ; 

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut tup = (nums[0].abs(), nums[0]);


        for val  in nums { 
            let dist = val.abs();
            // if the 
            if dist <  tup.0 { 
                tup.0 = dist;
                tup.1 = val;

            }else if dist  == tup.0  {
                if tup.1 < val { 
                    tup.1 = val
                }
            }
        }

        tup.1

    }
}
