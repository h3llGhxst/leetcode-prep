package medium

func removeCoveredIntervals(intervals [][]int) int {
	count := 0 

	for i,inter := range intervals { 

		for j := range intervals {
			if i == j {
				continue
			}
			in := intervals[j]
			l1, l2 := inter[0], in[0]
			r1, r2 := inter[1], in[1]
			if l2 <= l1 && r1 <= r2 {
				count++
				break
			}
		}
	}
	return len(intervals) - count
}
