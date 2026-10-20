package medium

type ListNode struct {
    Val int
    Next *ListNode
}

func swapNodes(head *ListNode, k int) *ListNode {

	from_list := toArray(head)

	left := k - 1
	right := len(from_list) - k

	temp := from_list[left]
	from_list[left] = from_list[right]
	from_list[right] = temp

	return toLinkedlist(from_list)

}

func toArray(head *ListNode ) []int { 
	var res []int
	curr := head

	for curr != nil { 
		res = append(res, curr.Val)
		curr  = curr.Next
	}

	return res
}

func toLinkedlist(arr []int ) *ListNode { 
	dummy  := &ListNode{}
	curr := dummy

	for _ , val := range arr { 
		curr.Next = &ListNode{Val: val}
		curr = curr.Next
	}
	return dummy.Next
}
