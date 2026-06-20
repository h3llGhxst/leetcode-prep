struct Solution;

impl Solution {
    pub fn k_items_with_maximum_sum(num_ones: i32, num_zeros: i32, num_neg_ones: i32, k: i32) -> i32 {
    let ones = num_ones.min(k);
    let mut sum = ones;
    let mut remaining = k - ones;

    let zeros = num_zeros.min(remaining);
    remaining -= zeros;        // zeros add 0, so sum doesn't change

    sum -= remaining;          // anything still left must be -1s
    sum
}
}
