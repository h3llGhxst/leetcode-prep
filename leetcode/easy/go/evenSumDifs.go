package go_test

func countPartitions(nums []int) int {
	count := 0 
	for i := 0 ; i < len(nums) -1 ; i++ { 
		if (sum(nums[0:i+1]) - sum(nums[i+1:])) % 2 == 0 { 
			count++
		} 
	}

	return count
}

func sum(a []int) int { 
	sum := 0 
	for _ , val := range a { 
		sum += val
	}
	return sum
}
