package go_test

import "strconv"

func findColumnWidth(grid [][]int) []int {
    n, m := len(grid), len(grid[0])
    res := make([]int, m)

    for j := range m {
        maxWidth := 0

        for i := range n {
            width := len(strconv.Itoa(grid[i][j]))
            if width > maxWidth {
                maxWidth = width
            }
        }

        res[j] = maxWidth
    }

    return res
}
