use super::ListNode;

// question 82
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut sentinel = Some(Box::new(ListNode::new(-1)));

    let mut prev = &mut sentinel;
    let mut tmp = &head;

    loop {
        if tmp.is_none() {
            prev.as_mut().unwrap().next = None;
            break;
        }

        let mut fast = &tmp.as_ref().unwrap().next;
        let mut is_dup = false;

        while let Some(v) = &fast {
            if v.val == tmp.as_ref().unwrap().val {
                fast = &v.next;
                is_dup = true;
            } else {
                break;
            }
        }

        if !is_dup {
            prev.as_mut().unwrap().next = Some(Box::new(ListNode::new(tmp.as_ref().unwrap().val)));
            prev = &mut prev.as_mut().unwrap().next;
        }
        tmp = fast;
    }

    sentinel.unwrap().next
}
