struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut counter = 0;

        for (i, &num) in nums.iter().enumerate() {
            let mut x = num;

            if i % 2 == 0 {
                while !Self::is_prime(x) {
                    x += 1;
                    counter += 1;
                }
            } else {
                while Self::is_prime(x) {
                    x += 1;
                    counter += 1;
                }
            }
        }

        counter
    }

    pub fn is_prime(n: i32) -> bool {
        if n <= 1 {
            return false;
        }

        if n == 2 {
            return true;
        }

        if n % 2 == 0 {
            return false;
        }

        for i in (3..=((n as f64).sqrt() as i32)).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }

        true
    }
}
