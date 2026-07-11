
struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut  moves_freq : HashMap<char, i32> =  HashMap::new(); 
        let mut max  = 0 ; 

        for c in moves.chars() {
            *moves_freq.entry(c).or_insert(0) += 1; 
        }
        let r = moves_freq.get(&'R').unwrap_or(&0) + moves_freq.get(&'_').unwrap_or(&0);
        let l = moves_freq.get(&'L').unwrap_or(&0) + moves_freq.get(&'_').unwrap_or(&0);

        max =  l.max(r);

        max - l.min(r) as i32
    }
}
