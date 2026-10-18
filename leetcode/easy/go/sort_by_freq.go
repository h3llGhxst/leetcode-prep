package go_test


import "sort"

func frequencySort(nums []int) []int {
	freq := make(map[int]int)
	for _, v := range nums {
		freq[v]++
	}

	sort.Slice(nums, func(i, j int) bool {
		a, b := nums[i], nums[j]
		if freq[a] != freq[b] {
			return freq[a] < freq[b] // fewer occurrences first
		}
		return a > b // tie: larger value first
	})
	return nums
}
