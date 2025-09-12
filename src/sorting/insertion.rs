pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j-1] > arr[j] {
            arr.swap(j-1, j);
            j -= 1;
        }
    }
}

pub fn insertion_sort_cloned<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let temp = arr[i].clone();
        let mut j = i;

        while j > 0 && arr[j-1] > temp {
            arr[j] = arr[j-1].clone();
            j -= 1;
        }

        arr[j] = temp;
    }
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

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr2, vec![1, 2, 3, 6]);
        assert_eq!(arr3, vec![1, 2, 3, 4, 7]);
    }

    #[test]
    pub fn insertion_sort_various() {
        let mut arr: Vec<i32> = Vec::new();
        let mut arr2 = vec![1];

        insertion_sort(arr.as_mut_slice());
        insertion_sort(arr2.as_mut_slice());

        assert_eq!(arr, vec![]);
        assert_eq!(arr2, vec![1]);
    }
}
