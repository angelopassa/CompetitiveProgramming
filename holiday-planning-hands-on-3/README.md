# Holiday Planning

In this exercise, we use a `dp` matrix of size $(n + 1) \times (D + 1)$ - where $n$ just the number of cities while $D$ are the number of days - in which at position $(i, \;j)$ we store the optimal solution for $j$ days and only taking the first $i$ cities into account.

The local solution for $(i, j)$ is shown below. Essentially if the number of cities is zero or if we have zero days available obviously the number of attractions visited in both cases is $0$.

Otherwise, the optimal solution for $(i, j)$ will depend on the number of days we want to stay in the city $i$: since it will have to be a number less than $j$, for each $k$ less or equal than $j$ we calculate the number of attractions visited if we stay in $i$ for $k$ days which will be given by the sum of all days prior to $k$ (inclusive). It will then be necessary to add to this value the maximum number of attractions visited in the first $i - 1$ cities and in $j - k$ days. Of all these sums, as $k$ varies, we take the maximum.

$$
M[i, \;j] = \begin{cases}
    0 & \text{if}\;\; i = 0 \;\lor\; j = 0 \\
    \underset{0 \leq k \leq j}{max}(M[i - 1, \;j - k] + \underset{0 \leq h < k}{\sum}S[h]) & \text{otherwise}
\end{cases}
$$

To make the sum of all days prior to $k$, fixed, effectively, instead of computing it for every $k$ we use a variable (`prefix_sum`) representing the prefixed sum up to $k$ of the attractions for the $i$-th city.

The code consists of three nested for-cycles, the first has cost exactly $n + 1$, the second has cost $D + 1$, while the third has cost at most $D$.

The cost is therefore $O(nD^2)$.