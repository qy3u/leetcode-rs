// question 155
pub struct MinStack {
    inner: Vec<i32>,
    min_helper: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            inner: vec![],
            min_helper: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        self.inner.push(x);
        if self.min_helper.is_empty() || x < *self.min_helper.last().unwrap() {
            self.min_helper.push(x);
        } else {
            self.min_helper.push(*self.min_helper.last().unwrap());
        }
    }

    fn pop(&mut self) {
        self.inner.pop();
        self.min_helper.pop();
    }

    fn top(&self) -> i32 {
        *self.inner.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_helper.last().unwrap()
    }
}
