#[derive(Debug)]
pub struct ListNode {
    pub val: i32,   // a value like i32
    pub next: Option<Box<ListNode>>,    // and a pointer to the next node (or None if it's the last)
    // In Rust, because of ownership rules we usually use Box(pointer like structure) to allocate the next node on the heap
}

#[derive(Debug)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,    // Option Because it can be None(empty)
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }   // new creates an empty LinkedList (head is None).  
    }

    pub fn push_front(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val, 
            next: self.head.take(),
        }) ;

        self.head = Some(new_node) ;
    }

    pub fn print_list(&self) {
        let mut current = self.head.as_ref() ;

        while let Some(node) = current {
            print!("{} -> ", node.val) ;
            current = node.next.as_ref() ;
        }

        println!("None") ;
    }
}


fn main() {
    println!("Hello From Ryana!") ;
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    list.print_list(); // Output: 1 -> 2 -> 3 -> None
}
