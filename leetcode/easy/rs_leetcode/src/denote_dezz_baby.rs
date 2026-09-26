
struct Solution;

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let n = bombs.len();
        // adjacency list: adj[i] = bombs that i directly detonates
        let mut adj = vec![vec![]; n];
        for i in 0..n {
            let (xi, yi, ri) = (bombs[i][0] as i64, bombs[i][1] as i64, bombs[i][2] as i64);
            for j in 0..n {
                if i == j { continue; }
                let dx = xi - bombs[j][0] as i64;
                let dy = yi - bombs[j][1] as i64;
                if dx * dx + dy * dy <= ri * ri {
                    adj[i].push(j); // directed: i → j only
                }
            }
        }

        let mut best = 0;
        for start in 0..n {
            let mut seen = vec![false; n];
            let mut stack = vec![start];
            seen[start] = true;
            let mut count = 0;
            while let Some(u) = stack.pop() {
                count += 1;
                for &v in &adj[u] {
                    if !seen[v] {
                        seen[v] = true;
                        stack.push(v);
                    }
                }
            }
            best = best.max(count);
        }
        best
    }
}
