struct  Solution;

impl Solution {
    pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::with_capacity(num.len() + 1);
        let mut carry = k;

        // walk num right-to-left, folding k in as the initial carry
        for &d in num.iter().rev() {
            carry += d;
            res.push(carry % 10);
            carry /= 10;
        }

        // whatever is left of k spills into new leading digits
        while carry > 0 {
            res.push(carry % 10);
            carry /= 10;
        }

        res.reverse();
        res
    }
}
