# Code Explanation

We use a `maxs` vector to store all the maximum elements of each window. The `deque` vector will be used to store the candidate elements as maxes for subsequent windows; to be precise, we'll have all the elements, in order as in the original vector, between the current window's momentary maximum and the current element being analysed. As we will see at the head of the deque we will always have the maximum element for that window.

The index `i` takes the current element into account, while the index `j` always represents the first element of the current window.

Within the `for` loop we discard all elements smaller than the current one from the deque, this is because for the current window it may be one of the maximum candidates and also for subsequent ones as the window moves forward so if the previous ones are smaller we can automatically discard them.

Once the current element is inserted in the deque we calculate the size of the current window (`c`). If this corresponds to `k`, then we can go to `maxs` to enter the maximum element for that window which will be at the top of the deque since we will always eliminate smaller numbers which are to the left of a larger number.

Finally, as the window will only move one cell, we will increment the index `j` by one, and if the element at the top of the deque corresponds to the first element of the current window, we will delete it as it cannot be considered for the next window. All other elements, however, will be taken into account, since we again point out that the window only moves one unit to the right, so they will belong to this one.

# Complexity Analysis

The `for` loop is executed exactly $n$ times. Within each iteration, however, we may go through an arbitrary number of element deletions (less than `k` in any case). We can see, however, that at the end of the `for` loop each element of the vector will be inserted and deleted from the deque no more than once. So the total cost given by the sum of the cost of all the `while` loops executed in each iteration will necessarily be linear.
