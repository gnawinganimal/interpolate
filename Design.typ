

Consider some function $f : FF -> V(FF)$.

We would like to be able to define a smooth function

#block($
  0 |-> (1, )
$)

Suppose $x_0, x_1, ..., x_k$ are scalars and $bold(y)_0, bold(y)_1, ..., bold(y)_k$ are vectors such that $L(x_j) = bold(y)_j$ for $0 <= j <= k$. We would like a function of some kind which "interpolates" these points.

#block($
  L(x) = sum_(j = 0)^k bold(y)_j ell_j (x) "where" ell : FF -> FF
$)

Wow, that's a lot.

With each pair $(x_j, bold(y)_j)$ we can associate a basis function $ell_j : FF -> FF$
