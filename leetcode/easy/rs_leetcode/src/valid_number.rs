
struct Solution;

impl Solution {
    pub fn valid_digit(mut n: i32, x: i32) -> bool {
        let original = n;
        let mut found = false;

        // Handle n = 0
        if n == 0 {
            found = x == 0;
        }

        while n > 0 {
            if n % 10 == x {
                found = true;
            }
            n /= 10;
        }

        // Find the leading digit
        let mut first = original;
        while first >= 10 {
            first /= 10;
        }

        found && first != x
    }
}
