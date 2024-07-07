use chapter01::interface::Queue;
use std::{cmp, mem};

#[derive(Default, PartialEq, Debug)]
pub struct ArrayQueue<T> {
    a: Box<[Option<T>]>,
    n: usize,
    j: usize,
}

impl<T> ArrayQueue<T> {
    pub fn capacity(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }
    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            n: 0,
            j: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(cmp::max(self.n * 2, 1));
        let mut old_a = mem::replace(&mut self.a, new_a);

        for k in 0..self.n {
            self.a[k] = old_a[(self.j + k) % old_a.len()].take();
        }
        self.j = 0;
    }
}

impl<T: Clone> Queue<T> for ArrayQueue<T> {
    fn add(&mut self, x: T) {
        if self.n + 1 > self.capacity() {
            self.resize();
        }

        self.a[(self.j + self.n) % self.capacity()] = Some(x);
        self.n += 1;
    }

    fn remove(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }

        let x = self.a[self.j].take();
        self.j = (self.j + 1) % self.capacity();
        self.n -= 1;

        if self.capacity() > self.n * 3 {
            self.resize();
        }

        x
    }
}

#[cfg(test)]
mod test {
    use super::ArrayQueue;
    use chapter01::interface::Queue;

    #[test]
    fn test_new() {
        let mut array_queue: ArrayQueue<char> = ArrayQueue::new();

        assert_eq!(array_queue.j, 0);
        assert_eq!(array_queue.remove(), None);
    }

    #[test]
    fn test_add() {
        let mut array_queue: ArrayQueue<char> = ArrayQueue::new();
        array_queue.add('a');
        assert_eq!(array_queue.j, 0);
        assert_eq!(array_queue.size(), 1);
        assert_eq!(array_queue.capacity(), 1);

        array_queue.add('b');
        array_queue.add('c');
        assert_eq!(array_queue.size(), 3);
        assert_eq!(array_queue.capacity(), 4);
    }

    #[test]
    fn test_remove() {
        let mut array_queue: ArrayQueue<char> = ArrayQueue::new();

        for c in "abcdef".chars() {
            array_queue.add(c);
        }
        array_queue.remove();
        array_queue.remove();
        array_queue.remove();
        array_queue.remove();

        assert_eq!(array_queue.capacity(), 4);
    }
}
