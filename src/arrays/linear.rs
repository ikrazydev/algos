use std::{ops::{Index, IndexMut}, ptr};

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

    pub fn remove_at(&mut self, index: usize) -> T {
        if self.len <= index {
            panic!("Index is out of range");
        }
        if self.len == 0 {
            panic!("No elements to remove");
        }

        // Unsafe code is required for returning the removed element
        unsafe {
            let result = ptr::read(self.arr.as_ptr().add(index));

            for i in index+1..self.len {
                self.arr.swap(i, i-1); // O(N)
            }

            self.len -= 1;
            result
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.arr.get(index) // O(1)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.arr.get_mut(index) // O(1)
    }

    pub fn len(&self) -> usize {
        self.len // O(1)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { list: self, index: 0 }
    }
}

impl<T: PartialEq> LinearArr<T> {
    pub fn index_of(&self, elem: &T) -> Option<usize> {
        self.arr.iter().position(|t| elem == t) // O(N)
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

pub struct IntoIter<T> {
    arr: LinearArr<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.arr.is_empty() {
            return None;
        }
        Some(self.arr.remove_at(0)) // O(N)
    }
}

impl<T> IntoIterator for LinearArr<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { arr: self }        
    }
}

pub struct Iter<'a, T> {
    list: &'a LinearArr<T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.list.len() {
            self.index += 1;
            return self.list.get(self.index-1); // O(1)
        }

        None
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
