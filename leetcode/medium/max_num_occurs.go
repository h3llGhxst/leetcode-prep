package medium


func maxFreq(s string, maxLetters int, minSize int, maxSize int) int {
    freqs := make(map[string]int)
    max_occur := 0

    for i := 0; i <= len(s)-minSize; i++ {
        sub := s[i : i+minSize]

        if uniques(sub) <= maxLetters {
            freqs[sub]++
            max_occur = max(max_occur, freqs[sub])
        }
    }

    return max_occur
}

func uniques(s string) int {
    seen := [26]bool{}
    count := 0

    for i := 0; i < len(s); i++ {
        idx := s[i] - 'a'

        if !seen[idx] {
            seen[idx] = true
            count++
        }
    }

    return count
}
