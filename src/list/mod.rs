mod remove_duplicate_from_sorted_list_ii;
mod remove_nth_node_from_end_of_list;
mod rotate_list;
mod swap_nodes_in_pairs;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn create_node_by_order(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for i in vals.into_iter().rev() {
        let mut node = Box::new(ListNode::new(i));
        if head.is_none() {
            head = Some(node);
        } else {
            node.next = head;
            head = Some(node);
        }
    }

    head
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut tmp = head;
    println!("[");
    while let Some(node) = tmp {
        println!("{}", node);
        tmp = &node.next;
    }
    println!("]");
}

use std::fmt;
impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.val)
    }
}

pub use remove_duplicate_from_sorted_list_ii::delete_duplicates;
pub use remove_nth_node_from_end_of_list::remove_nth_from_end;
pub use rotate_list::rotate_right;
pub use swap_nodes_in_pairs::swap_pairs;
