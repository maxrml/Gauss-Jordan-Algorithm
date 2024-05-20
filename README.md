# Gauss-Jordan-Algorithm

# How can you implement the Gauss-Jordan-Algorithm to bring a matrix in their row echelon form?

With this program I want to offer a quite easy way to do so, since there is not the one right algorithm,
there are many ways to accomplish the Gauss-Elimination. My way might not be the best in terms of time complexity
[O(m * n * log(n))] but for the sake of general complexity "coding-difficulty-like" this might be a good choice!

# The idea is quite simple and works in a few steps:
1. The Matrix, represented by a Vector of i32 Vectors, gets sorted such that rows with the more zeros in
the front are lower sorted.
2. Starting at row 1, you iterate through the row and look for the first element != 0 (Pivot Element)
3. You search in the lower rows if in row 2, 3... there are also pivot elements at the same space
if yes: eliminate the pivot elements in the lower rows by elementary line transformation
if not: go a row lower and search for the pivot element and continue

This process goes on till you reach the last line of the matrix!
