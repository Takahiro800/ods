use chapter01::interface::List;
use std::{cmp, fmt::Debug, mem};

#[derive(Default, PartialEq, Debug)]
pub struct ArrayDeque<T> {
    a: Box<[Option<T>]>,
    n: usize,
    j: usize,
}

impl<T> ArrayDeque<T> {
    pub fn capacity(&self) -> usize {
        self.a.len()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            a: Self::allocat_in_heap(capacity),
            n: 0,
            j: 0,
        }
    }

    fn allocat_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocat_in_heap(cmp::max(self.n * 2, 1));
        let mut old_a = mem::replace(&mut self.a, new_a);

        for k in 0..self.n {
            self.a[k] = old_a[(self.j + k) % old_a.len()].take();
        }
        self.j = 0;
    }
}

impl<T: Clone + Debug> List<T> for ArrayDeque<T> {
    fn size(&self) -> usize {
        todo!()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i > self.n {
            return None;
        }

        self.a
            .get((i + self.j) % self.capacity())?
            .as_ref()
            .cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut((self.j + i) % self.capacity())?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if i > self.n {
            return;
        }

        if self.size() + 1 > self.capacity() {
            self.resize();
        }

        if i < self.size() / 2 {
            self.j = if self.j == 0 {
                self.capacity() - 1
            } else {
                self.j - 1
            };

            for k in 0..i {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k + 1) % self.capacity()].take();
            }
        } else {
            for k in (i + 1..self.size()).rev() {
                self.a[(self.j + k) % self.capacity()] =
                    self.a[(self.j + k - 1) % self.capacity()].take();
            }
        }
        self.a[(self.j + i) % self.capacity()] = Some(x);
        self.n += 1;
        // println!("{:?}", self.a);
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.size() {
            return None;
        }

        let x = self.a[(self.j + i) % self.capacity()].take();

        if i < self.size() / 2 {
            for k in (0..i).rev() {
                self.a[(self.j + k + 1) % self.capacity()] =
                    self.a[(self.j + k) % self.capacity()].take();
            }
            self.j = (self.j + 1) % self.capacity();
        } else {
            for k in (i..self.size()).rev() {
                self.a[(self.j + k + 1) % self.capacity()] =
                    self.a[(self.j + k) % self.capacity()].take();
            }
        }
        self.n -= 1;

        if self.size() * 3 < self.capacity() {
            self.resize();
        }

        x
    }
}

#[cfg(test)]
mod test {
    use super::ArrayDeque;
    use chapter01::interface::List;

    #[test]
    fn test_new() {
        let mut array_deque: ArrayDeque<i32> = ArrayDeque::new();
        assert_eq!(array_deque.size(), 0);
        assert_eq!(array_deque.capacity(), 1);
        assert_eq!(array_deque.remove(0), None);
        assert_eq!(array_deque.remove(5), None);
    }

    #[test]
    fn test_add() {
        let mut array_deque: ArrayDeque<i32> = ArrayDeque::new();

        array_deque.add(0, -10);
        array_deque.add(3, 3);
        assert_eq!(array_deque.get(0), Some(-10));
        assert_eq!(array_deque.get(3), None);
    }

    #[test]
    fn test_remove() {
        let mut array_deque: ArrayDeque<i32> = ArrayDeque::new();

        array_deque.add(0, -10);
        array_deque.add(1, 10);
        array_deque.add(2, -20);
        array_deque.add(3, 30);
        array_deque.add(4, -40);
        array_deque.add(5, 50);
        assert_eq!(array_deque.get(0), Some(-10));
        assert_eq!(array_deque.get(3), Some(30));

        array_deque.remove(2);
        assert_eq!(array_deque.get(2), Some(30));
    }
}
