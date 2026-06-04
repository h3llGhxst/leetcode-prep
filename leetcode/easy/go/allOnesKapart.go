package go_test

func kLengthApart(nums []int, k int) bool {
	count_zeros := 0
	seen := false

	for _, val := range nums {
		if val == 0 {
			count_zeros++
		} else if val == 1 {
			if seen && count_zeros < k {
				return false
			}
			seen = true
			count_zeros = 0
		}
	}

	return true
}
