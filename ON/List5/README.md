In this List we need to modify the gauss elimination method to solve custom matrix faster than with normal gauss elimination method.

# TODO:
- [x] Analyse normal gauss elimination method
- [ ] Analyse custom matrix's structure

# Notes:
- Gauss elimination method not only simplifies the matrix to triangular matrix, but also computes *A = LU*.
- In exercise we need to make 2 variants:
    - with out changing rows
    - with changing rows to chose the biggest element in column
- First exercise is to find *x* in *Ax = b*.
- Second exercise is to find *LU* in *A = LU*.
- Third exercise is to function that from *LU* and *b* finds *x* in *Ax = b*, which means *LUx = b*.
    - if second variant, change rows as in *A*.
    - compute *y* in *Ly = b*.
    - compute *x* in *Ux = y*.
