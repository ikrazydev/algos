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

Funnily enough, there's an even better optimization.
The `bubble_sort_optimized` function utilizes the "last swap" technique.
It can be thought of as an extension to "the largest unsorted element is at the end" rule.

Instead of sorting one less element each time, you essentially "check" how many elements are *already* sorted.
All of the elements after the last swap have already been sorted (since they didn't get swapped),
so we don't have to go through any of them next time. 
That lets you shrink your sorting field by (potentially) multiple elements each pass, instead of just one,
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

That is also why it's considered to be the simplest sorting algorithm.
Easy to teach, but even with its optimizations, is still largely inferior to more performant algorithms.

# Selection Sort

Selection sort starts at the first element. The algorithm then finds the lowest element after it.
If they're not the same elements, the two are swapped in places.
Since the first element is already sorted, the algorithm moves onto the next element and performs the same procedure.

As a result, the amount of steps in the lowest-element-searching loop decrements with each iteration.

That makes for an interesting phenomenon. Even though sorting algorithms, like bubble sort,
have optimizations where the algorithm can be stopped once it figured out the entire array is already sorted,
selection sort, on the other hand, always performs the same amount of steps.
The only difference is the swap may not be performed if the current and lowest elements are the same.

Since after each iteration, the amount of steps to perform decreases by 1,
the amount of steps is approximately half of $\ N^2 $, or $\ O(N^2/2) $.
However, in Big O, it is still categorized as $\ O(N^2) $ (and thus equivalent to bubble sort),
since the notation disregards constants.

From that, we can derive the time complexities of the algorithm:

- **Worst Case**: *around half of* $\ O(N^2) $
- **Average Case**: *around half of* $\ O(N^2) $
- **Best Case**: *around half of* $\ O(N^2) $

In actual notation, they're just considered $\ O(N^2) $.

In most scenarios, even with an inferior best case, selection sort is still going to be perform considerably faster
than its pal, bubble sort. That fact is self-evident from the workings of the algorithms.

In general, selection sort does not swap elements unnecessarily,
which seems to be the core weakness of bubble sorting.
You can only benefit from bubble sorting if your arrays are mostly sorted,
and the algorithm is optimized properly.
Unfortunately, for most use cases, that usually is not the situation.

# Insertion Sort

TODO.
