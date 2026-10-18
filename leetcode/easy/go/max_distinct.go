package go_test


import "slices"

func maxKDistinct(nums []int, k int) []int {
	slices.Sort(nums) 
	nums = slices.Compact(nums)

	if k > len(nums) {        
		k = len(nums)
	}

	res := slices.Clone(nums[len(nums)-k:])
	slices.Reverse(res)      
	return res
}
