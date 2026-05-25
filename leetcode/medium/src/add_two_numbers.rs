 #[derive(PartialEq, Eq, Clone, Debug)]

 pub struct ListNode {
   pub val: i32,
   pub next: Option<Box<ListNode>>
 }
 
 impl ListNode {
   #[inline]
   fn new(val: i32) -> Self {
    ListNode {
      next: None,
       val
     }
   }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  
        let mut p1 = l1 ; 
        let mut p2 = l2 ; 
        let mut carry = 0 ;

        let mut dummy = Box::new(ListNode::new(0)); 
        let mut tail = &mut dummy;

        while p1.is_some() || p2.is_some() || carry > 0 { 

            let (v1, next1) = if let Some(n) = p1.take() { 
                (n.val, n.next)
            } else { 
                (0, None)
            };

            let (v2, next2) = if let Some(n) = p2.take() { 
                (n.val, n.next)
            } else { 
                (0, None)
            };

            p1 = next1; 
            p2 = next2; 

            let sum = v1 +  v2 + carry; 
            carry = sum / 10 ;

            tail.next = Some(Box::new(ListNode::new(sum % 10))); 
            tail = tail.next.as_mut().unwrap();
        }

        dummy.next
    }
}
