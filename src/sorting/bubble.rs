pub fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut swapped = false;

        for j in 1..arr.len()-i { // O(N^2)

            if arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                swapped = true;
            } 

        }

        if !swapped {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::bubble::*;

    #[test]
    fn bubble_sort_test() {
        let mut arr = vec![5, 3, 4, 1, 2];
        bubble_sort(arr.as_mut_slice());

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
