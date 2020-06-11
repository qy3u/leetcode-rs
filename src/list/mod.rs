mod remove_nth_node_from_end_of_list;
mod swap_nodes_in_pairs;

#[derive(Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub use remove_nth_node_from_end_of_list::remove_nth_from_end;
pub use swap_nodes_in_pairs::swap_pairs;
