// question 61

// 1. question
// Given a linked list, rotate the list to the right by k places, where k is non-negative.

// Example 1:
//
// Input: 1->2->3->4->5->NULL, k = 2
// Output: 4->5->1->2->3->NULL
// Explanation:
// rotate 1 steps to the right: 5->1->2->3->4->NULL
// rotate 2 steps to the right: 4->5->1->2->3->NULL
//
// Example 2:
//
// Input: 0->1->2->NULL, k = 4
// Output: 2->0->1->NULL
// Explanation:
// rotate 1 steps to the right: 2->0->1->NULL
// rotate 2 steps to the right: 1->2->0->NULL
// rotate 3 steps to the right: 0->1->2->NULL
// rotate 4 steps to the right: 2->0->1->NULL

// 2. my solution

use super::*;

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let len = get_list_length(&head);
    if len <= 1 {
        return head;
    }

    let k = k as usize % len;

    let mut head = head;
    for _ in 0..k {
        head = rotate_once(head, len);
    }
    head
}

fn get_list_length(head: &Option<Box<ListNode>>) -> usize {
    let mut tmp = head;
    let mut n = 0;
    while let Some(node) = tmp {
        n += 1;
        tmp = &node.next;
    }
    n
}

fn rotate_once(head: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
    // assert the length of head is equal to len
    // assert len >= 2
    let mut head = head;
    let mut tmp = &mut head;

    let mut before_end = len - 2;

    let mut tail = None;
    while let Some(node) = tmp {
        if before_end == 0 {
            tail = node.next.take();
            break;
        }
        tmp = &mut node.next;
        before_end -= 1;
    }

    match tail.as_mut() {
        None => unreachable!(),
        Some(node) => {
            node.next = head;
        }
    }
    tail
}

// 3. other's solution
// class Solution {
// public:
//     ListNode* rotateRight(ListNode* head, int k) {
//         if(!head) return head;

//         int len=1; // number of nodes
//         ListNode *newH, *tail;
//         newH=tail=head;

//         while(tail->next)  // get the number of nodes in the list
//         {
//             tail = tail->next;
//             len++;
//         }
//         tail->next = head; // circle the link

//         if(k %= len)
//         {
//             for(auto i=0; i<len-k; i++) tail = tail->next; // the tail node is the (len-k)-th node (1st node is head)
//         }
//         newH = tail->next;
//         tail->next = NULL;
//         return newH;
//     }
// };
//
// public ListNode rotateRight(ListNode head, int n) {
//     if (head==null||head.next==null) return head;
//     ListNode dummy=new ListNode(0);
//     dummy.next=head;
//     ListNode fast=dummy,slow=dummy;

//     int i;
//     for (i=0;fast.next!=null;i++)//Get the total length
//     	fast=fast.next;

//     for (int j=i-n%i;j>0;j--) //Get the i-n%i th node
//     	slow=slow.next;

//     fast.next=dummy.next; //Do the rotation
//     dummy.next=slow.next;
//     slow.next=null;

//     return dummy.next;
// }
//
//
// 4. what can be improved
//  1) 方法:
//      构造成环再前进, 最后断开
//      直接算出合适的位置

// 5. improve again
//    note: 这里定义的 Node 在 rust 里是无法构造环的
//

pub fn rotate_right_again(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let len = get_list_length(&head);
    if len <= 1 {
        return head;
    }

    let k = k as usize % len;

    let head = rotate_n(head, len, k as usize);

    head
}

fn rotate_n(head: Option<Box<ListNode>>, len: usize, n: usize) -> Option<Box<ListNode>> {
    if n == 0 {
        return head;
    }

    let mut head = head;
    let mut tmp = &mut head;

    let mut before_end_n = len - n - 1; // the node before(points) the nth node

    let mut new_head = None;
    while let Some(node) = tmp {
        if before_end_n == 0 {
            new_head = node.next.take();
            break;
        }
        tmp = &mut node.next;
        before_end_n -= 1;
    }

    tmp = &mut new_head;
    while let Some(ref mut node) = tmp {
        if node.next.is_none() {
            node.next = head;
            break;
        }
        tmp = &mut node.next;
    }

    new_head
}

// 6. thinking
//  1) 考虑一种思路的时候要理性的分析可行性:
//      rust 在 Option<Box<ListNode>> 下是无法定义成环的
//  2) 要对一个固定的 function 考虑边界条件
//      起码要考虑 0 和 len
//
