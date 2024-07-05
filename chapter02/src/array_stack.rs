use std::{cmp::max, mem};

use chapter01::interface::List;

#[derive(Default, PartialEq, Debug)]
pub struct Array<T> {
    a: Box<[Option<T>]>,
    n: usize,
}

impl<T> Array<T> {
    pub fn length(&self) -> usize {
        self.a.len()
    }

    pub fn new() -> Self {
        Self::with_length(1)
    }

    pub fn with_length(capacity: usize) -> Self {
        Self {
            a: Self::allocate_in_heap(capacity),
            n: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[Option<T>]> {
        std::iter::repeat_with(Default::default)
            .take(size)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn resize(&mut self) {
        let new_a = Self::allocate_in_heap(max(self.n * 2, 1));
        let old_a = mem::replace(&mut self.a, new_a);

        for (i, v) in old_a.into_vec().into_iter().enumerate() {
            self.a[i] = v;
        }
    }
}

impl<T: Clone> List<T> for Array<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<T> {
        self.a.get(i)?.as_ref().cloned()
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        self.a.get_mut(i)?.replace(x)
    }

    fn add(&mut self, i: usize, x: T) {
        if self.n + 1 > self.length() {
            self.resize();
        }

        if i >= self.n {
            self.a[self.n] = Some(x);
        } else {
            self.a[i..self.n].rotate_right(1);

            let end = self.a[i].replace(x);
            self.a[self.n] = end;
        }
        self.n += 1;
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        let x = self.a.get_mut(i)?.take();

        if i < self.n {
            self.a[i..self.n].rotate_left(1);
            self.n -= 1;

            if self.length() >= self.n * 3 {
                self.resize();
            }
        }
        x
    }
}

#[cfg(test)]
mod test {
    use super::Array;
    use chapter01::interface::List;

    #[test]
    fn test_array_stack() {
        let mut stack: Array<i32> = Array::new();
        assert_eq!(stack.size(), 0);

        stack.add(0, 1);
        assert_eq!(stack.get(0), Some(1));

        stack.add(3, 100);
        assert_eq!(stack.get(3), None);
        assert_eq!(stack.get(1), Some(100));
        assert_eq!(stack.size(), 2);

        assert_eq!(stack.remove(1), Some(100));
        assert_eq!(stack.size(), 1);
        assert_eq!(stack.remove(2), None);

        stack.add(1, 200);
        stack.add(2, 300);
        stack.set(1, 10);
        assert_eq!(stack.get(1), Some(10));
    }
}
