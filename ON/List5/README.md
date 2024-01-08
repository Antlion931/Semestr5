In this List we need to modify the gauss elimination method to solve custom matrix faster than with normal gauss elimination method.

# TODO:
- [x] Analyse normal gauss elimination method
- [x] Analyse custom matrix's structure
- [x] Make a function to read data from file
    - [x] Before reading data, you need a data structure to store it
- [x] Try to simplify the matrix 12x12 on paper
- [x] Write *Ax = b* solver
    - [x] To make it faster write new functions to MOC, to get index of last meaningful element in rows
    - [x] To make it faster, change swap to don't swap, but changes pointers to rows
    - [x] Split MOC into MOC and MOC with selection
    - [x] Write it without selection
    - [x] Write it with selection
    - [x] Write Zero near zero check
- [ ] Write *A = LU* solver\
    - [x] There is some error, it doesn't produce ones
    - [ ] Wrong swap, It is not changing all possible rows, skips all from B when in A
    - [ ] L and swaps needs different interpretation
    - [ ] Make it work with partial selection
- [ ] Write *LUx = b* solver

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
- Our app needs to do 2 things:
    - computes *Ax = b*
    - computes *A = LU*, and then *LUx = b*
    To achieve this we need data, there are 2 ways to get it:
    - read *A* and *b* from file, result should be *x'*
    - read *A* and compute *b* from where *x = `[1, 1, ..., 1]`*, result should be *x'*, and error to real *x*. 
    Additionaly there should be a way to store results in file.

