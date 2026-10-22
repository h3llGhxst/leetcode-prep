
package medium

type TreeNode struct {
    Val int
    Left *TreeNode
    Right *TreeNode
}

func replaceValueInTree(root *TreeNode) *TreeNode {
    if root == nil {
        return nil
    }

    root.Val = 0
    queue := []*TreeNode{root}

    for len(queue) > 0 {
        levelSize := len(queue)
        levelSum := 0

        // First pass: sum the values of all children
        // and add them to the queue.
        for i := range levelSize {
            node := queue[i]

            if node.Left != nil {
                levelSum += node.Left.Val
                queue = append(queue, node.Left)
            }

            if node.Right != nil {
                levelSum += node.Right.Val
                queue = append(queue, node.Right)
            }
        }
	
        // Second pass: update each child's value.
        for i :=  range levelSize {
            node := queue[i]

            siblingSum := 0

            if node.Left != nil {
                siblingSum += node.Left.Val
            }

            if node.Right != nil {
                siblingSum += node.Right.Val
            }

            if node.Left != nil {
                node.Left.Val = levelSum - siblingSum
            }

            if node.Right != nil {
                node.Right.Val = levelSum - siblingSum
            }
        }

        // Move to the next level.
        queue = queue[levelSize:]
    }

    return root
}

