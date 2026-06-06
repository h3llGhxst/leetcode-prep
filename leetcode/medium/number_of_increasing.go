package medium


func findNumberOfLIS(nums []int) int {
	n := len(nums) 

	length := make([]int , n) //longets subseq.
	count := make([]int , n) //how many they are.

	for i := range n { 
		length[i] = 1
		count[i] = 1

		for j := range i  { 
			
			if nums[j] < nums[i]{ 
				if length[j] + 1 > length[i] { 
					length[i] = length[j] + 1
					count[i] = count[j]
				} else if length[j]+1 == length[i] { 
					count[i] +=count[j]
				}
			}
		}
	}
	
	maxLen := 0 

	for i := range n { 
		if length[i] > maxLen { 
			maxLen = length[i]
		}
	}

	results := 0 

	for i := range n { 
		if length[i] == maxLen { 
			results += count[i]
		}
	}

	return results

}
