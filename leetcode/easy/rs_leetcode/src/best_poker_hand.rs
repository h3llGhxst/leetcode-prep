struct Solution;


use std::collections::HashMap;

impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let mut freq_rank: HashMap<i32, i32> = HashMap::new();

        for rank in ranks {
            *freq_rank.entry(rank).or_insert(0) += 1;
        }

        let flush = suits.iter().all(|&s| s == suits[0]);

        if flush {
            return String::from("Flush");
        }

        let rank_res = *freq_rank.values().max().unwrap();

        if rank_res >= 3 {
            String::from("Three of a Kind")
        } else if rank_res == 2 {
            String::from("Pair")
        } else {
            String::from("High Card")
        }
    }
}

