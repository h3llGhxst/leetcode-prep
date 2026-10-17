package go_test


func projectionArea(grid [][]int) int {

	var ans = 0 
	n := len(grid)

	for i := range n {  // assume n * n as said i was trying to accom mr
		maxRow := 0 
		maxColumn := 0 

		for j := range n {
			if grid[i][j] != 0 { 
				ans += 1
			}

			if grid[i][j] > maxRow { 
				maxRow = grid[i][j]
			}
			if grid[j][i] > maxColumn { 
				maxColumn = grid[j][i]
			}
		}
		ans += maxColumn 
		ans += maxRow
	}
	return ans
}
