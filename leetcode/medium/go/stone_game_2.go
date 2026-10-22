package medium


func stoneGameII(piles []int) int {

	n := len(piles)

	suffix := make([]int, n)
	suffix[n-1] = piles[n-1]

	for i := n - 2; i >= 0; i-- {
		suffix[i] = suffix[i+1] + piles[i]
	}

	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n+1)
	}

	var solve func(i, m int) int

	solve = func(i, m int) int {
		if 2*m >= n-i {
			return suffix[i]
		}

		if memo[i][m] != 0 {
			return memo[i][m]
		}

		best := 0

		for x := 1; x <= 2*m; x++ {
			opponent := solve(i+x, max(m, x))
			stones := suffix[i] - opponent

			if stones > best {
				best = stones
			}
		}

		memo[i][m] = best
		return best
	}

	return solve(0, 1)
}
