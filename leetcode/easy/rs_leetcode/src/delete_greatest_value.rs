impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for row in grid.iter_mut() {
            row.sort_unstable();
        }

        let cols = grid[0].len();
        let mut answer = 0;

        for col in 0..cols {
            let mut round_max = 0;
            for row in &grid {
                round_max = round_max.max(row[col]);
            }
            answer += round_max;
        }

        answer
    }
}
