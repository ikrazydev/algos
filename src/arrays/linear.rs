use std::ops::{Index, IndexMut};

pub struct LinearArr<T> {
    arr: Vec<T>,
    len: usize,
}

impl<T> LinearArr<T> {
    pub fn new() -> Self {
        Self { arr: Vec::new(), len: 0 }
    }

    pub fn new_with(arr: Vec<T>) -> Self {
        let len = arr.len();
        Self { arr, len }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        if index > self.len {
            panic!("Index can't be above length");
        }

        self.arr.push(elem); // temporarily add a new element
        self.len += 1;

        for i in (index+1..self.len).rev() {
            self.arr.swap(i, i-1); // O(N)
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.arr.get(index) // O(1)
    }

    pub fn len(&self) -> usize {
        self.len // O(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: PartialEq> LinearArr<T> {
    pub fn index_of(&self, elem: &T) -> Option<usize> {
        self.arr.iter().position(|t| elem == t) // O(N)
    }

    pub fn remove_at(&mut self, index: usize) {
        if self.len <= index {
            panic!("Index is out of range");
        }
        if self.len == 0 {
            panic!("No elements to remove");
        }

        for i in index+1..self.len {
            self.arr.swap(i, i-1); // O(N)
        }

        self.len -= 1;
        self.arr.pop();
    }

    pub fn remove(&mut self, elem: &T) {
        if self.len == 0 {
            panic!("No elements to remove");
        }
        let Some(index) = self.index_of(elem) else {
            panic!("Element is not in array");
        };

        self.remove_at(index);
    }
}

impl<T> Index<usize> for LinearArr<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.arr.index(index)
    }
}

impl<T> IndexMut<usize> for LinearArr<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.arr.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::linear::*;

    #[test]
    fn insert_test() {
        let mut arr = LinearArr::new();
        arr.insert(0, 0); arr.insert(0, 1);

        assert_eq!(arr.index_of(&1), Some(0));
        assert_eq!(arr.index_of(&0), Some(1));

        arr.insert(1, 2); arr.insert(1, 3);
        assert_eq!(arr.index_of(&1), Some(0));
        assert_eq!(arr.index_of(&2), Some(2));
        assert_eq!(arr.index_of(&3), Some(1));
        assert_eq!(arr.index_of(&0), Some(3));
    }

    #[test]
    fn remove_test() {
        let mut arr = LinearArr::new_with(vec![1, 2, 3]);
        arr.remove(&1); arr.remove(&2); arr.remove(&3);

        assert_eq!(arr.len, 0);
    }
}
