# Bubble Sort

Bubble sorting uses multiple iterations over the entire array in order to sort it. It checks the two neighboring numbers one by one.
If the preceding number is larger than the next, the two elements are swapped in their places.

One iteration over the array is not enough, since sorting multiple elements requires multiple passes.
It is only guaranteed that the next largest unsorted element will be sorted properly each iteration.
The "bubble" in the name refers to that exact phenomenon.
The largest unsorted element "bubbles" up to the end.

Why is that? Well, since the largest element is larger than any other element (duh),
it will always be swapped with any element that is next to it.
That makes the element go towards the end, since it literally has no obstacles!
Result: The largest element is pushed to the end.

This also means that with every iteration you need to sort one less element, which gives you some room for optimization.

Funnily enough, there's an even better solution.
The `bubble_sort_optimized` function utilizes the "last swap" optimization technique.
It can be thought of as an extension to "the largest unsorted element is at the end" rule.

Instead of sorting one less element each time, you essentially "check" how many elements are *already* sorted.
All of the elements after the last swap have already been sorted (since they didn't get swapped),
meaning we don't have to go through any of them next time. 
That lets you shrink your sorting field by multiple elements each pass, instead of just one,
thus greatly improving performance.
Knowing your algorithms well makes a big difference.

Altogether, it makes the time complexity of this primitive sorting algorithm to be N * N, which is $\ O(N^2) $ in Big O notation.

More specifically,

- **Worst Case**: $\ O(N^2) $
- **Average Case**: *around* $\ O(N^2) $
- **Best Case**: $\ O(N) $

The provided implementation uses some optimizations (early return, passes decreasing in length),
however its time complexity is still considered to be $\ O(N^2) $.
That demonstrates why Big O isn't the only (nor the best, in many situations) way to describe the efficiency of an algorithm.

# Selection Sort

TODO.
