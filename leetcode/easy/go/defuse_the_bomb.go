package go_test


func decrypt(code []int, k int) []int {
    n := len(code)
    res := make([]int, n)
    if k == 0 {
        return res
    }

    lo, hi := 1, k
    if k < 0 {
        lo, hi = n+k, n-1
    }

    // initial sum
    sum := 0
    for j := lo; j <= hi; j++ {
        sum += code[j%n]
    }

    for i := 0; i < n; i++ {
        res[i] = sum
		
        sum -= code[lo%n]
        lo++
        hi++
        sum += code[hi%n]
    }
    return res
}
