
package medium

func longestSemiRepetitiveSubstring(s string) int {

	max_sub_str := 1

	for i := 0  ; i < len(s) ; i++ { 

		
		for j := i+1 ; j < len(s); j++ { 
			sub_str := s[i:j+1]

			if semi_rep(sub_str) { 
				if len(sub_str) > max_sub_str { 
					max_sub_str = len(sub_str)
				}
			}

		}
	}
	
	return max_sub_str
}

func semi_rep(stri string) bool  { 
	counter := 0 


	for i := 1 ; i < len(stri) ; i++  {

		if stri[i] == stri[i-1] { 
			counter++
		}
		if counter > 1 { 
			return false
		}
	}

	return true
}
