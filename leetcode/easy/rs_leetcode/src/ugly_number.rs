
struct Solution; 

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let targets: &[i32] = &[2, 3, 5];

        for &factor in targets {
            while n % factor == 0 {
                n /= factor;
            }
        }

        n == 1
    }
}
