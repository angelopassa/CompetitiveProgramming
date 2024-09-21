# Code Explanation

We use 4 indices, `max_l` to indicate the index of the block of maximum height from the left, while `max_r` will indicate the index of the block of maximum height but from the right. The indices `i` and `j` will represent the blocks analysed so far, from the left and right respectively, then the following properties will apply:
- `i` $\geq$ `max_l`
- `j` $\leq$ `max_r`

Thus, we start from the left with the index `0` and from the right with the last index of the vector, i.e. `height.len() - 1`.

What is done within the `while` loop is to calculate the amount of water for each index by going forwards or backwards depending on where the smallest height is. This is correct because the property holds that the amount of water that can be stored is definitely less than or equal to the minimum between the maximum height on the left and the maximum height on the right. So if, for example, the smallest value between `height[max_l]` and `height[max_r]` is on the left, we don't care if there are taller blocks on the right than index `i` since the maximum height between `0` and `i` is precisely `height[max_l]` which corresponds to the maximum amount of water for index `i`.

What is then done in both branches of the first `if` is to check whether the height of the index in question, `i` or `j`, is less than the minimum value at position `max_l`, or `max_r` as the case may be. If so, the `sum` accumulator is incremented by the correct difference between the heights of the two indices being considered.

# Complexity Analysis

The cost of the algorithm is linear ($O(n)$) since the `while` loop examines each cell of the vector exactly once (when `i` and `j` become equal the loop is exited).