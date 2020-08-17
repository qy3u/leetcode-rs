// question 23
use super::ListNode;

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    if lists.is_empty() {
        return None;
    }

    let mut lists = lists;
    let mut heap = BinaryHeap::new();

    loop {
        let mut is_all_none = true;
        for i in 0..lists.len() {
            if lists[i].is_some() {
                heap.push(Reverse(lists[i].as_ref().unwrap().val));
                lists[i] = lists[i].as_mut().unwrap().next.take();
                is_all_none = false;
            }
        }

        if is_all_none {
            break;
        }
    }

    if heap.is_empty() {
        return None;
    }

    let mut result = ListNode::new(heap.pop().unwrap().0);

    let mut tmp: &mut ListNode = &mut result;

    while !heap.is_empty() {
        tmp.next = Some(Box::new(ListNode::new(heap.pop().unwrap().0)));
        tmp = tmp.next.as_mut().unwrap();
    }

    Some(Box::new(result))
}
