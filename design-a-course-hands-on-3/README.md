# Design a course

The solution to this exercise is inspired by the solution to the [**Longest Increasing Sequence**](https://www.geeksforgeeks.org/longest-increasing-subsequence-dp-3/#using-binary-search-on-log-n-time-and-on-space) problem but in time $n \log n$ using binary search.

The first step is to order the original array that has cost $O(n \log n)$. We then instantiate an array that will have length equal to the number of maximum topics that form a strictly increasing order on both *difficulty* and *beauty* values.

The strategy is to insert a value at the end of this list if it is greater than the last, otherwise it is inserted in the position following its first predecessor in the order. Since the array is by nature ordered the cost of searching for the predecessor and replacing its successor with the current element has cost $\Theta(\log n)$.

The correctness of this strategy is ensured by the fact that the array is sorted on the pair *(beauty, difficulty)* using the standard tuple comparison, i.e:

$$
(x_1, \;y_1) \leq (x_2, \;y_2) \;\Longleftrightarrow\; x_1 \leq x2 \;\land\; (x_1 = x_2 \;\Longrightarrow\; y_1 \leq y_2) 
$$

Thus the element taken into consideration by the `for` loop, called `t`, will always have a greater *beauty* value and will therefore always be the next element of a **Increasing Sequence** (not necessarily the longest).

The number of elements preceding `t` in an *Increasing Sequence* is equal to the length of the subarray from position $0$ to the position of its predecessor in the solution array.

This is because the array in the `for` loop is scanned according to the original position of the elements so if the `sequence` array reaches length $n$ then there will surely have been a sequence $a_i, \;b_j, \;\dots, \;c_h$ in the past (where the indices represent the position in the original array) such that $i < j < \dots < h$. Even if one or more of those elements have now been replaced by others that succeed them.

This last property is very important as it guarantees that, although the **Longest Increasing Sequence** may not be in the `sequence` array, its length will definitely be the same.

A small limiting case had to be handled: that is, in the case where the element whose place $t$ will take in `sequence` has the same value as *beauty*, $t$ is inserted in its place only if it has a lower *difficulty* value. This works because in the ordering imposed by the trace, the **Longest Increasing Subsequence** must have no repeated values of either field.

The final complexity is therefore $O(n \log n)$.