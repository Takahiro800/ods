use super::array_stack::ArrayStack;
use chapter01::interface::List;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct DualArrayDeque<T: Clone> {
    front: ArrayStack<T>,
    back: ArrayStack<T>,
}

impl<T: Clone> DualArrayDeque<T> {
    pub fn capacity(&self) -> usize {
        self.front.capacity() + self.back.capacity()
    }

    pub fn new() -> Self {
        Self {
            front: ArrayStack::new(),
            back: ArrayStack::new(),
        }
    }

    pub fn balance(&mut self) {
        if 3 * self.front.size() >= self.back.size() && 3 * self.back.size() >= self.front.size() {
            return;
        }

        let mid = self.size() / 2;

        let nf = mid;
        let mut new_front: ArrayStack<T> = ArrayStack::with_capacity(std::cmp::max(2 * nf, 1));
        for i in 0..mid {
            new_front.add(nf - i - 1, self.remove(0).unwrap());
        }

        let nb = self.size() - nf;
        let mut new_back: ArrayStack<T> = ArrayStack::with_capacity(std::cmp::max(2 * nb, 1));
        for i in 0..nb {
            new_back.add(i, self.remove(0).unwrap());
        }

        self.front = new_front;
        self.back = new_back;
    }
}

impl<T: Clone> List<T> for DualArrayDeque<T> {
    fn size(&self) -> usize {
        self.front.size() + self.back.size()
    }

    fn get(&self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.get(self.front.size() - i - 1)
        } else {
            self.back.get(i - self.front.size())
        }
    }

    fn set(&mut self, i: usize, x: T) -> Option<T> {
        if i < self.front.size() {
            self.front.set(self.front.size() - i - 1, x)
        } else {
            self.back.set(i - self.front.size(), x)
        }
    }

    fn add(&mut self, i: usize, x: T) {
        if i < self.front.size() {
            self.front.add(self.front.size() - i - 1, x)
        } else {
            self.back.add(i - self.front.size(), x)
        }
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i < self.front.size() {
            self.front.remove(self.front.size() - i - 1)
        } else {
            self.back.remove(i - self.front.size())
        }
    }
}

#[cfg(test)]
mod test {
    use super::DualArrayDeque;
    use chapter01::interface::List;

    #[test]
    fn test_new() {
        let deque: DualArrayDeque<i32> = DualArrayDeque::new();
        assert_eq!(deque.size(), 0);
    }

    #[test]
    fn test_add() {
        let mut deque: DualArrayDeque<i32> = DualArrayDeque::new();
        assert_eq!(deque.size(), 0);

        deque.add(0, 1);
        assert_eq!(deque.get(0), Some(1));

        deque.add(3, 30);
        assert_eq!(deque.get(3), None);
        assert_eq!(deque.get(1), Some(30));
    }

    #[test]
    fn test_remove() {
        let mut deque: DualArrayDeque<i32> = DualArrayDeque::new();
        assert_eq!(deque.size(), 0);

        deque.add(0, 1);
        assert_eq!(deque.get(0), Some(1));

        deque.add(3, 30);
        assert_eq!(deque.get(1), Some(30));

        assert_eq!(deque.remove(1), Some(30));
        assert_eq!(deque.remove(1), None);
    }
}
