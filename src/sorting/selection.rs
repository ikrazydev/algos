pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut lowest = i;

        for j in i+1..arr.len() { // O(N^2)
            if arr[j] < arr[lowest] {
                lowest = j;
            }
        }

        if lowest != i {
            arr.swap(i, lowest);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::selection::*;

    #[test]
    pub fn selection_sort_test() {
        let mut arr = vec![5, 3, 4, 1, 2];
        let mut arr2 = vec![2, 6, 1, 3];
        let mut arr3 = vec![4, 2, 7, 1, 3];

        selection_sort(arr.as_mut_slice());
        selection_sort(arr2.as_mut_slice());
        selection_sort(arr3.as_mut_slice());

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr2, vec![1, 2, 3, 6]);
        assert_eq!(arr3, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    pub fn selection_sort_various() {
        let mut arr: Vec<i32> = Vec::new();
        let mut arr2 = vec![1];

        selection_sort(arr.as_mut_slice());
        selection_sort(arr2.as_mut_slice());

        assert_eq!(arr, vec![]);
        assert_eq!(arr2, vec![1]);
    }
}
