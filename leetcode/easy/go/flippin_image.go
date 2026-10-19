package go_test

import "slices"

func flipAndInvertImage(image [][]int) [][]int {

	for j, v := range image { 

		slices.Reverse(v)
		for i , val := range v { 
			if val == 1 { 
				v[i] = 0
			}else{ 
				v[i] = 1
			}
		}

		image[j] = v
	}

	return image
}
