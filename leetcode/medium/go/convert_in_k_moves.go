package medium

func canConvertString(s string, t string, k int) bool {
    if len(s) != len(t) { 
		return false
	} 

	var used [26]int 

	for i := 0 ; i < len(s) ; i ++ { 
		shift := (int(t[i]) - int(s[i]) + 26) % 26 

		if shift == 0 { 
			continue
		}

		move := shift + 26*used[shift]

		if move > k { 
			return false
		}
		used[shift]++
	} 
	return true
}
