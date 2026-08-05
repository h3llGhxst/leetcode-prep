
struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let neg = (dividend < 0) != (divisor < 0); 

        let mut  rem = (dividend as i64).abs();
        let div = (divisor as i64).abs();
        let mut quot : i64 = 0;

        for k in (0..32).rev()  {
            if (div << k)  <= rem { 
                rem -= div << k;
                quot += 1 << k 
            }
        }

        let singned  = if neg {
            -quot
        } else {
            quot
        };

        singned.clamp(i32::MIN as i64, i32::MAX as i64) as i32
    }
}
