package medium
func maxSum(nums []int, m int, k int) int64 {
	cnt := map[int]int{} 
	var sum, best int64

	for i, v := range nums {
		cnt[v]++
		sum += int64(v)

		if i >= k {
			out := nums[i-k]
			cnt[out]--
			if cnt[out] == 0 {
				delete(cnt, out)
			}
			sum -= int64(out)
		}

		if i >= k-1 && len(cnt) >= m {
			best = max(best, sum)
		}
	}
	return best
}
