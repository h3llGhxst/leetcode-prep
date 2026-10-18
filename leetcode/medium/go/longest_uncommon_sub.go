package medium


func findLUSlength(strs []string) int {

	best := -1 

	for i , s:= range strs { 
		unique := true

		for j , t := range strs { 
			if i != j && isSub(s , t) { 
				unique = false
				break 
			}
		}

		if unique { 
			best = max(best , len(s))
		}

	}

	return best
}


func isSub(s , t string) bool { 
	p := 0 

	for i := 0 ; i < len(t) && p < len(s) ; i ++ { 
		if t[i] == s[p] { 
			p++ 
		}
	}

	return  p == len(s)
}
