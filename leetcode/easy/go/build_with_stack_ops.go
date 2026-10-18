

package go_test

func buildArray(target []int, n int) []string {

	res := []string {}
	t := 0 

   for num := 1 ; num <= n && t < len(target) ; num ++ { 
	   res = append(res, "Push")

	   if num == target[t] { 
		   t++
	   }else { 
		   res = append(res, "Pop")
	   }
   }

   return res
}
