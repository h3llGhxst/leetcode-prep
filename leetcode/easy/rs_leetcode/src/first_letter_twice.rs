struct Solution; 
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut seen: u32 = 0;
        for ch in s.chars() {
            let bit = 1u32 << (ch as u32 - 'a' as u32);
            if seen & bit != 0 {
                return ch;
            }
            seen |= bit;
        }
        unreachable!()
    }
}
