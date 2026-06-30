struct Solution;

impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut cost = cost; 
        cost.sort_unstable_by(|a , b| b.cmp(a));

        cost.iter()
            .enumerate()
            .filter(|(i, _)| (i +1) % 3 != 0)
            .map(|(_, &c)| c)
            .sum()
    }
}
