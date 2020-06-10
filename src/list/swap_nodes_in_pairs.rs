// question 24
use super::ListNode;

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    if let Some(head_box) = &head {
        if head_box.next.is_none() {
            return head;
        }
    }

    let mut first = head.unwrap();
    let mut second = first.next.take().unwrap();
    let third = second.next.take();
    let third = swap_pairs(third);

    first.next = third;
    second.next = Some(first);
    return Some(second);
}
