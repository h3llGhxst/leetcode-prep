package go_test

func maxSum(nums []int) int {
    digits := [10]int{}

    for i := range digits {
        digits[i] = -1
    }

    ans := -1

    for _, num := range nums {
        largest := largestDigit(num)

        if digits[largest] != -1 {
            sum := digits[largest] + num
            ans = max(ans, sum)
        }

        if num > digits[largest] {
            digits[largest] = num
        }
    }

    return ans
}

func largestDigit(num int) int {
    maxi := 0

    for num > 0 {
        maxi = max(maxi, num%10)
        num /= 10
    }

    return maxi
}

