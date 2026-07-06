struct Solution; 

use std::collections::VecDeque;
impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
        for i in 0..n - 1 {
            adj[i].push(i + 1);
        }

        let mut answer = Vec::with_capacity(queries.len());

        for q in queries {
            let (u, v) = (q[0] as usize, q[1] as usize);
            adj[u].push(v);
            answer.push(Self::bfs(&adj, n));
        }

        answer
    }

    fn bfs(adj: &Vec<Vec<usize>>, n: usize) -> i32 {
        let mut dist = vec![-1; n];
        dist[0] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(0);

        while let Some(node) = queue.pop_front() {
            if node == n - 1 {
                return dist[node];
            }
            for &next in &adj[node] {
                if dist[next] == -1 {
                    dist[next] = dist[node] + 1;
                    queue.push_back(next);
                }
            }
        }

        dist[n - 1]
    }
}
