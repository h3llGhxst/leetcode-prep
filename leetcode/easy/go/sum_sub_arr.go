package go_test

func maxLength(nums []int) int {
	longest := 0

	for i := range nums {
		prod := 1
		g := 0
		l := 1

		for j := i; j < len(nums); j++ {
			val := nums[j]

			prod *= val
			g = my_gcd(g, val)
			l = l / my_gcd(l, val) * val

			if prod == l*g {
				length := j - i + 1
				if length > longest {
					longest = length
				}
			}
		}
	}

	return longest
}

func my_gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

