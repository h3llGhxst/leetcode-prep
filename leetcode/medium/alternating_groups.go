package medium

func numberOfAlternatingGroups(colors []int) int {

	counter := 0 
	n := len(colors)

	//first and last element are troube some (avoid stupid circles.)
	
	//if this is true add to counter , min len is 3 ..i thnk this covers it  
	if colors[0] != colors[1] && colors[0] != colors[n-1] { 
		counter++
	}

	//do same for he last element

	last := colors[n-1] 
	if last != colors[0] && last != colors[n-2] { 
		counter++
	}

	//now do the middle easy sliding window ones
	
	for i := 1 ; i < len(colors) - 1 ; i++ { 
		if colors[i] != colors[i-1] && colors[i] != colors[i+1] { 
			counter ++
		}
	}

	return counter
}

//worked  first try btw
