package medium

func minSwaps(s string) int {

	balance := 0 
	minBalance := 0 

	for _ , b := range s { 
		switch b { 
			case ']' : 
				balance--
				minBalance = min(minBalance  , balance)
			case '[' :
				balance ++
				//this does not minimise anything
		}
	}

	return (-minBalance +1 ) / 2
}
