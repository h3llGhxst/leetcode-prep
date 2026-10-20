package medium

func minFlips(grid [][]int) int {
	
	from_cols := fix_cols(grid) 
	from_rows:= fix_rows(grid) 

	if from_cols <= from_rows { 
		return from_cols
	}
	return from_rows
}

func fix_cols(grid [][] int ) int { 

	no_of_swaps  := 0 

	for col := 0; col < len(grid[0]); col++ {
    	i, j := 0, len(grid)-1

    	for i < j {
        	if grid[i][col] != grid[j][col] {
				no_of_swaps++
        	}
        	i++
        	j--
    	}
	}
	return no_of_swaps
}


func fix_rows(grid [][] int) int { 
	no_of_swaps := 0 
	//i dont even have to do the swapping ... count faults

	for _  , row := range grid { 
		i  , j := 0 , len(row)-1
		for i < j { 
			if row[i] != row[j] { 
				no_of_swaps++
			} 
			//continue gracefuly. 
			i++ 
			j-- 
		}
	} 
	return no_of_swaps
}
