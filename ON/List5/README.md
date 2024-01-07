In this List we need to modify the gauss elimination method to solve custom matrix faster than with normal gauss elimination method.

# TODO:
- [x] Analyse normal gauss elimination method
- [x] Analyse custom matrix's structure
- [ ] Make a function to read data from file
    - [ ] Before reading data, you need a data structure to store it
- [x] Try to simplify the matrix 12x12 on paper

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
- This is how our custom matrix looks like:
```
A A A A C
A A A A   C
A A A A     C
A A A A       C
      B A A A A C
      B A A A A   C
      B A A A A     C
      B A A A A       C
              B A A A A
              B A A A A
              B A A A A
              B A A A A
```

- Possible improvment is to subtract from every row on threads
