
struct Solution;

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut last = [0usize; 3]; // last house index for M, P, G

        for (i, s) in garbage.iter().enumerate() {
            total += s.len() as i32; // pickup: 1 min per unit, truck doesn't matter

            for b in s.bytes() {
                match b {
                    b'M' => last[0] = i,
                    b'P' => last[1] = i,
                    _    => last[2] = i, // only 'G' is left
                }
            }
        }

        for &l in &last {
            total += travel[..l].iter().sum::<i32>(); // drive from house 0 to house l
        }

        total
    }
}
