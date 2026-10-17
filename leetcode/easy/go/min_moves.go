package go_test

import "slices"

func minMoves(nums []int) int {
	ans := 0
	maxi := slices.Max(nums)
	for _ , val := range nums { 
		ans += maxi - val
	}
	return ans
}
