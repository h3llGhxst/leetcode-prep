struct Solution;
impl Solution {

    fn max_product(nums: Vec<i32>) -> i32 {
    let mut max1 = 0; // largest
    let mut max2 = 0; // second largest

    for n in nums {
        if n > max1 {
            max2 = max1; // old champ slides down to runner-up
            max1 = n;    // new champ
        } else if n > max2 {
            max2 = n;    // not the best, but beats runner-up
        }
    }

    (max1 - 1) * (max2 - 1)
}

}
