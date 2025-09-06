# Bubble Sort

Bubble sorting uses multiple iterations over the entire array in order to sort it. It checks the two neighboring numbers.
If the preceding number is larger than the next, the two elements are swapped in their places.

One iteration over the array is not enough, since sorting multiple consecutive elements requires multiple passes.
It is only guaranteed that the next largest unsorted element will be sorted properly each iteration.
This also means that every iteration you need to sort one less element, which is room for optimization.

Altogether, it makes the time complexity of this primitive sorting algorithm to be N * N, which is $\ O(N^2) $ in Big O notation.

The provided implementation uses some optimizations (early return, decreasing array length),
however its time complexity is still considered to be $\ O(N^2) $.
That demonstrates why Big O isn't the only (nor best, in many situations) way to describe the efficiency of an algorithm.

# Selection Sort

TODO.