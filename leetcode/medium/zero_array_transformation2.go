
package medium



func minZeroArray(nums []int, queries [][]int) int {
    if isZeroArray(nums) {
        return 0
    }

    // Even all queries might not be enough.
    if !canMakeZero(nums, queries, len(queries)) {
        return -1
    }

    // Binary search for the minimum number of queries.
    lo, hi := 1, len(queries)

    for lo < hi {
        mid := lo + (hi-lo)/2

        if canMakeZero(nums, queries, mid) {
            hi = mid
        } else {
            lo = mid + 1
        }
    }

    return lo
}

func canMakeZero(nums []int, queries [][]int, k int) bool {
    n := len(nums)
    diff := make([]int, n+1)

    for i := range k {
        l, r, val := queries[i][0], queries[i][1], queries[i][2]

        diff[l] += val
        diff[r+1] -= val
    }

    capacity := 0

    for i, num := range nums {
        capacity += diff[i]

        if capacity < num {
            return false
        }
    }

    return true
}

func isZeroArray(nums []int) bool {
    for _, num := range nums {
        if num != 0 {
            return false
        }
    }
    return true
}

