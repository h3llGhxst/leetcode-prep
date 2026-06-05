package medium

func minIncrease(nums []int) int64 {
	n := len(nums)

	cost := func(i int) int64 {
		need := nums[i-1]
		if nums[i+1] > need {
			need = nums[i+1]
		}
		need++
		if need > nums[i] {
			return int64(need - nums[i])
		}
		return 0
	}

	// dp: maximize count, then minimize ops
	// skip = best result when we don't take index i
	// take = best result when we do take index i
	skipC, skipO := 0, int64(0)
	takeC, takeO := -1, int64(0)

	for i := 1; i < n-1; i++ {
		nsC, nsO := skipC, skipO
		if takeC > nsC || (takeC == nsC && takeO < nsO) {
			nsC, nsO = takeC, takeO
		}
		takeC, takeO = skipC+1, skipO+cost(i)
		skipC, skipO = nsC, nsO
	}

	if takeC > skipC || (takeC == skipC && takeO < skipO) {
		return takeO
	}
	return skipO
}
