# Linear Array

Linear arrays implement most operations in O(N) complexity.
Since memory is stored linearly, access and modification times are O(1).

- **Insertion**: O(N)
- **Deletion**: O(N)
- **Searching**: O(N)
- **Index Retrieval/Modification**: O(1)

# Ordered Array

Ordered arrays give the ability to utilize binary search.
It uses ordered numbers as an advantage, where it starts with the middle,
and then chooses either to go left (if value is lower than the middle), or right (if it's higher).
Once it finds the right elements, it's done (which, yes, can also be the middle of the array).
It's impossible to do that on non-sorted arrays.

Binary search is a O(log N) algorithm,
which is a significant improvement over O(N) linear searching that linear arrays are capable of.
Since each step essentially reduces the searching field twofold,
we can think of it as reversing the exponent (if we start with 8, it becomes 4, then 2, 1).
That's the definition of logarithm with base 2!

You don't need to specify the base of the logarithm, hence O(log N).

Ordered arrays are a limitation on normal linear arrays,
so the rest of the operations take the same time as they would on a linear array.

# Hash Tables

Hash tables are essentially arrays stored in buckets that have a key-value relationship.

## Hash Functions

There are perfect and imperfect hash functions.

TODO: hash collisions.

# Sets and Hash Sets

TODO.
