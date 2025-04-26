// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0) ; // Dummy node to start
        let mut current = &mut dummy_head ;
        let mut carry = 0 ;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let sum = 
                l1.as_ref().map_or(0, |node| node.val) +    // l1's value or 0
                l2.as_ref().map_or(0, |node| node.val) +    // l2's value or 0
                carry ;

                carry = sum / 10 ;
                let new_node = ListNode::new(sum % 10) ;    // current digit node
                current.next = Some(Box::new(new_node)) ;

                current = current.next.as_mut().unwrap() ;  

                // Move l1 and l2 to next 
                if let Some(node) = l1 { l1 = node.next; }
                if let Some(node) = l2 { l2 = node.next; }
        }

        dummy_head.next
    }
}