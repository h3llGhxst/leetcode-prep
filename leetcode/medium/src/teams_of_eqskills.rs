
struct Solution;
impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort_unstable();
        let n = skill.len(); 
        let target = skill[0] +skill[n-1];
        let mut total : i64 = 0; 
        let mut  l : usize = 0 ; 
        let mut r : usize = n -1 as usize;  

        while l < r {
            
            if skill[l] + skill[r] != target { 
                return -1; 
            }

            total += skill[l] as i64 * skill[r] as i64;
            l += 1; 
            r -= 1;
        }
        total
    }
}
