# Report HandsOn 1

## Binary Search Tree

The `is_bst()` method makes use of the auxiliary function `is_bst_priv(node_id)`, which takes as input the index of the node in the vector `nodes`, representing the root tree from the subtree for which the property is to be verified. Wanting of course to verify it for the whole tree, we pass as index `0` representing the root.

Now let us analyse the `is_bst_priv(node_id)` function. If the index of the passed node is `None`, it means that the parent of `node_id` that invoked the procedure has no left or right subtree. So an empty tree obviously respects the property of being a binary search tree, but since it contains no values, we set `MIN` as the *maximum* element in the left subtree and `MAX` as the *minimum* value in the right subtree.
If the `node_id` exists, it will have an associated `v` value representing its identifier, i.e. its index in the `nodes` vector. Now before computing the necessary information for the current subtree, we decide to use a *bottom-up* approach, and so make recursive calls first to the left subtree and then to the right subtree, obtaining their return values. For each subtree, we will therefore have the *minimum* and *maximum* element and a boolean value `is_bst_*` to indicate whether the subtree in question is a binary search tree. We now obtain the value of the current node via the node's `key` attribute in the vector.
We can now compute the information to be passed to the current node's parent.

The minimum in this subtree will be the minimum between the left subtree, the right subtree, and the current node joining the two subtrees. For the maximum, the procedure is symmetrical. Instead, to verify that the current subtree is a binary search tree, it is necessary:

- Firstly, that both subtrees are already BSTs.
- That the current node is greater than or equal to (let us assume that we always put the nodes with equal value in the left subtree) the maximum value in the left subtree.
- And that the value of the current node is also less than or equal to the minimum value found in the right subtree.

These three conditions combined make it possible to check whether the current subtree is a BST. Then returning to procedure `is_bst()` will return the last value of the triple, which will represent the boolean value associated with the complete tree.

## Maximum Path Sum
