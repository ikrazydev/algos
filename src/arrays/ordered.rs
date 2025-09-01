use crate::arrays::linear::LinearArr;

pub struct OrderedArr<T: PartialOrd> {
    pub arr: LinearArr<T>,
}

impl<T: PartialOrd> OrderedArr<T> {
    pub fn new() -> Self {
        Self { arr: LinearArr::new() }
    }

    fn _verify_sortedness(arr: &Vec<T>) -> bool {
        arr.windows(2).all(|w| w[0] <= w[1])
    }

    pub fn new_with(arr: Vec<T>) -> Self {
        if !Self::_verify_sortedness(&arr) {
            panic!("Can't create from an unsorted array");
        }

        Self { arr: LinearArr::new_with(arr) }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.arr.get(index) // O(1)
    }

    pub fn len(&self) -> usize {
        self.arr.len() // O(1)
    }

    fn _binary_search(&self, elem: &T) -> Result<usize, usize> {
        if self.arr.is_empty() {
            return Err(0);
        }

        let mut lower_bound = 0;
        let mut upper_bound = self.arr.len()-1;
        let mut middle;

        while lower_bound <= upper_bound {
            middle = lower_bound + (upper_bound - lower_bound) / 2;

            // O(log N)
            if self.arr[middle] == *elem {
                return Ok(middle);
            } else if self.arr[middle] > *elem {
                if middle > 0 {
                    upper_bound = middle - 1;
                } else {
                    break;
                }
            } else {
                lower_bound = middle + 1;
            }
        }

        Err(lower_bound)
    }

    pub fn insert(&mut self, elem: T) {
        let index = match self._binary_search(&elem) { // O(log N)
            Ok(index) => index,
            Err(index) => index,
        };

        self.arr.insert(index, elem); // O(N)
    }

    pub fn index_of(&self, elem: &T) -> Option<usize> {
        self._binary_search(elem).ok() // O(log N)
    }

    pub fn remove(&mut self, elem: &T) {
        if self.arr.is_empty() {
            panic!("No elements to remove");
        }
        let Some(index) = self.index_of(elem) else { // O(log N)
            panic!("Element is not in array");
        };

        self.arr.remove_at(index); // O(N)
    }
}

#[cfg(test)]
mod tests {
    use crate::arrays::ordered::*;

    #[test]
    pub fn index_of_tests() {
        let arr = OrderedArr::new_with(vec![1, 2, 3, 4]);

        assert_eq!(arr.index_of(&1), Some(0));
        assert_eq!(arr.index_of(&2), Some(1));
        assert_eq!(arr.index_of(&3), Some(2));
        assert_eq!(arr.index_of(&4), Some(3));
    }

    #[test]
    #[should_panic]
    pub fn sortedness_panic() {
        OrderedArr::new_with(vec![3, 1, 2]);
    }

    #[test]
    fn insert_test() {
        let mut arr = OrderedArr::new();
        arr.insert(1); arr.insert(0);

        assert_eq!(arr.index_of(&0), Some(0));
        assert_eq!(arr.index_of(&1), Some(1));

        arr.insert(2); arr.insert(3);
        assert_eq!(arr.index_of(&0), Some(0));
        assert_eq!(arr.index_of(&1), Some(1));
        assert_eq!(arr.index_of(&2), Some(2));
        assert_eq!(arr.index_of(&3), Some(3));
    }

    #[test]
    fn remove_test() {
        let mut arr = OrderedArr::new_with(vec![1, 2, 3]);
        arr.remove(&1); arr.remove(&2); arr.remove(&3);

        assert_eq!(arr.len(), 0);
    }
}
