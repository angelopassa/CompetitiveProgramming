## Segment Trees Representation

In both exercises, **Segment Trees** were represented as vectors $2n - 1$ long, as this is the exact amount of nodes in the tree. To avoid creating vectors $4n$ long, I decided to enumerate the nodes not as in a binary heap ($2i + 1$ for the left child and $2i + 2$ for the right), but following the order of an Euler tour traversal.

So if the parent has index $i$, the left child will be numbered with $i + 1$, the right child will have the number immediately following the last one used in the subtree with root the left child. In general, if the vector representing $i$ goes from $p$ to $q$ it will have $2(q - p + 1) - 1$ nodes, the left child of $i$ representing the subarray in the interval $[p, mid]$ (with mid equal to $\lfloor (q - p)/ 2 \rfloor$) will be $mid - p + 1$ long and consequently will have $2(mid - p + 1) - 1$ nodes. So the index associated with the right child of $i$ will be $i + 2(mid - p + 1)$. The $-1$ disappears because I have to take the next number. 

## Is There Problem

In this problem, since the maximum length of each segment is precisely $n$ and both extremes are between $0$ and $n - 1$, the idea is to create a **Segment Tree** on all possible points included in the segments and store for each point the number of segments containing it. If we have this information, we can store in each internal node of the segment tree, which will represent the interval $[l, r]$, the number of segments for each point in the interval. This is done by storing in each node a `HashSet`, i.e. a set, which contains precisely these numbers, thus indicating that for every number $x$ in the `HashSet`, in the interval $[l, r]$, there is at least one point contained in $x$ segments. This is sufficient to answer the requested query: no information is needed about the occurrence of each number or which point it belongs to.

The cost of the `is_there_inner()` procedure is that of a search query on the segment trees, i.e. $O(\log n)$. The combination of the result of the recursive call in the left half and the result of the right half is an *OR*, since we are interested in *at least* one point between $i$ and $j$ belonging to $k$ segments.

The problem that remains to be solved is how to calculate for each point the segments containing it. This is done with a **Sweep Line**. In fact, in the `load_segments()` procedure, we create our `sweep_line` vector of points $n + 1$ long, initialised at $0$ for all points, and for each segment $[l, r]$ we increment at position $l$ by $1$ to indicate that a new segment starts at that position, and at position $r + 1$ we decrement by $1$ to indicate that from this point the new segment no longer exists and therefore does not contain it (hence the vector is $n + 1$ long and the segment tree has $2n + 1$ nodes).

Finally, we calculate the prefix sums of the vector `sweep_line` to find the number of segments located at each point.

Once the prefix sum has been calculated for a fixed `point`, this number is inserted into the segment tree at the `point` position. This is done in the `add_point()` method, which performs a kind of binary search to find the path from the root to the leaf representing the `point`, and which will then have cost $O(\log n)$. All the nodes touched during this path will represent segments in which `point` is contained, and so we can put the number representing the segments in `point` into their `HashSet`.

Since the points are $n + 1$ the total cost of `load_segments()`, which is executed only once, is $O(n \log n)$.

Finally, having $m$ query *IsThere*, since the cost of each `is_there()` as already mentioned is $O(\log n)$, the total cost of the algorithm will be $O((n + m) \log n)$.