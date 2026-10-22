package medium

import ( "sort")

func kthSmallestPrimeFraction(arr []int, k int) []int {
	pairs := [][] int{} 

	for i := range arr { 
		for j := i+1; j < len(arr) ; j++ { 
			pairs = append(pairs,[]int{arr[i], arr[j]})
		}
	}

	return sortByCrossMul(pairs)[k-1]

}

func sortByCrossMul(arr [][] int) [][] int  { 
	sort.Slice(arr , func(i, j int) bool {
		a , b := arr[i][0] ,arr[i][1]
		c , d := arr[j][0] ,arr[j][1]

		return a*d < c*b
	})

	return arr
}


