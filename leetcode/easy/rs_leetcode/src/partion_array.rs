struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();

        if sum % 3 != 0 {
            return false;
        }

        let target = sum / 3;
        let mut current = 0;
        let mut parts = 0;

       for i in 0..arr.len() - 1 {
            current += arr[i];

            if current == target {
                parts += 1;
                current = 0;

                if parts == 2 {
                    return true;
                }
            }
        }

        false
    }
}
