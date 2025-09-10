pub fn insertion_sort<T: Ord>(arr: &mut [T]) {

}

#[cfg(test)]
mod tests {
    use crate::sorting::insertion::*;

    #[test]
    fn insertion_sort_test() {
        let mut arr = vec![5, 3, 4, 1, 2];
        let mut arr2 = vec![2, 6, 1, 3];
        let mut arr3 = vec![4, 2, 7, 1, 3];

        insertion_sort(arr.as_mut_slice());
        insertion_sort(arr2.as_mut_slice());
        insertion_sort(arr3.as_mut_slice());

        // assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        // assert_eq!(arr2, vec![1, 2, 3, 6]);
        // assert_eq!(arr3, vec![1, 2, 3, 4, 7]);
    }
}
