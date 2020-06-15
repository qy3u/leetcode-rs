// question 19
use super::ListNode;

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut tmp = &head;
    while let Some(node) = tmp {
        tmp = &node.next;
        len += 1;
    }
    let nth = len - n + 1;
    remove_nth(head, nth)
}

pub fn remove_nth(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    assert!(n > 0);
    if n == 1 {
        return head.unwrap().next;
    }

    let mut head = head.unwrap();
    let next = head.next;
    head.next = remove_nth(next, n - 1);

    Some(head)
}
