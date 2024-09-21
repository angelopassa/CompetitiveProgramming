# Code Explanation

We use 4 indices, `max_l` to indicate the index of the block of maximum height from the left, while `max_r` will indicate the index of the block of maximum height but from the right. The indices `i` and `j` will represent the blocks analysed so far, from the left and right respectively, then the following properties will apply:
- `i` $\geq$ `max_l`
- `j` $\leq$ `max_r`

Thus, we start from the left with the index `0` and from the right with the last index of the vector, i.e. `height.len() - 1`.

# Complexity Analysis