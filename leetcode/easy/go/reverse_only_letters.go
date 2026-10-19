package go_test
func reverseOnlyLetters(s string) string {
    chars := []byte(s)
    i, j := 0, len(chars)-1

    isLetter := func(c byte) bool {
        return ('a' <= c && c <= 'z') ||
            ('A' <= c && c <= 'Z')
    }

    for i < j {
        if !isLetter(chars[i]) {
            i++
            continue
        }

        if !isLetter(chars[j]) {
            j--
            continue
        }

        chars[i], chars[j] = chars[j], chars[i]
        i++
        j--
    }

    return string(chars)
}
