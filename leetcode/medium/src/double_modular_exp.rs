
struct Solution;
impl Solution {
    fn mod_pow(mut base: i32, mut exp: i32, modu: i32) -> i32 {
        let mut ans = 1;
        base %= modu;

        while exp > 0 {
            if exp % 2 == 1 {
                ans = (ans * base) % modu;
            }

            base = (base * base) % modu;
            exp /= 2;
        }

        ans
    }

    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let mut goodies = Vec::new();

        for (i, arr) in variables.iter().enumerate() {
            let a = arr[0];
            let b = arr[1];
            let c = arr[2];
            let m = arr[3];

            let first = Self::mod_pow(a, b, 10);
            let second = Self::mod_pow(first, c, m);

            if second == target {
                goodies.push(i as i32);
            }
        }

        goodies
    }
}
