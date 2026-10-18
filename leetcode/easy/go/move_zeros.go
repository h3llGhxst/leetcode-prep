package go_test

func moveZeroes(nums []int) {
	w := 0
	for r := range nums {
		if nums[r] != 0 {
			nums[w], nums[r] = nums[r], nums[w]
			w++
		}
	}
}  
