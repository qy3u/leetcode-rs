// #![feature(box_patterns)]
mod array;
mod list;
mod string;

use list::*;

fn main() {
    let second = ListNode { val: 2, next: None };
    let first = ListNode {
        val: 1,
        next: Some(Box::new(second)),
    };

    let list = swap_pairs(Some(Box::new(first)));

    println!("{:?}", list);
}
