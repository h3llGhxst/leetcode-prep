struct Solution;
impl Solution {
    pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
        let mut total: i64 = 0;
        let mut gains: Vec<i32> = Vec::new();

        for i in 0..technique1.len() {
            total += technique2[i] as i64;
            gains.push(technique1[i] - technique2[i]);
        }

        gains.sort_by(|a, b| b.cmp(a));

        for i in 0..k as usize {
            total += gains[i] as i64;
        }

        for i in k as usize..gains.len() {
            if gains[i] > 0 {
                total += gains[i] as i64;
            }
        }

        total
    }
}
