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

pub fn bubble_sort_optimized<T: PartialOrd>(arr: &mut [T]) {
    let mut n = arr.len();

    while n > 1 {
        let mut new_n = 0;

        for j in 1..n {
            if arr[j-1] > arr[j] {
                arr.swap(j-1, j);
                new_n = j;
            }
        }

        n = new_n; // next pass only needs to check up to last swap
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::bubble::*;

    #[test]
    fn bubble_sort_test() {
        let mut arr = vec![5, 3, 4, 1, 2];
        let mut arr2 = arr.clone();
        bubble_sort(arr.as_mut_slice());
        bubble_sort_optimized(arr2.as_mut_slice());

        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
        assert_eq!(arr, arr2);
    }
}
