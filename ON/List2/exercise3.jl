# Albert Kołodziejski
using LinearAlgebra

function matcond(n::Int, c::Float64)
# Function generates a random square matrix A of size n with
# a given condition number c.
# Inputs:
#	n: size of matrix A, n>1
#	c: condition of matrix A, c>= 1.0
#
# Usage: matcond(10, 100.0)
#
# Pawel Zielinski
        if n < 2
         error("size n should be > 1")
        end
        if c< 1.0
         error("condition number  c of a matrix  should be >= 1.0")
        end
        (U,S,V)=svd(rand(n,n))
        return U*diagm(0 =>[LinRange(1.0,c,n);])*V'
end

function hilb(n::Int)
    # Function generates the Hilbert matrix  A of size n,
    #  A (i, j) = 1 / (i + j - 1)
    # Inputs:
    #	n: size of matrix A, n>=1
    #
    #
    # Usage: hilb(10)
    #
    # Pawel Zielinski
            if n < 1
             error("size n should be >= 1")
            end
            return [1 / (i + j - 1) for i in 1:n, j in 1:n]
    end

function x(n)
    ones(n, 1)
end

function b(A, x)
    A * x
end

function first(A, b)
    A\b
end

function second(A, b)
    inv(A)*b
end

println("matrix & rank & cond & A\\b & A^-1 b")
for n = 2:20
    A = hilb(n)
    my_b = b(A, x(n))

    println("hilb(", n, ") & ", rank(A), " & ", round(cond(A); sigdigits = 4, base = 10), " & ", round(norm(first(A, my_b) - x(n)) / norm(x(n)); sigdigits = 4, base = 10), " & ", round(norm(second(A, my_b) - x(n))/ norm(x(n)); sigdigits = 4, base = 10), "\\\\")
end

println("Now random ==================")

for n = [5, 10 , 20]
    for c = [1.0, 10.0, 10.0^3, 10.0^7, 10.0^12, 10.0^16] 
        A = matcond(n, c)
        my_b = b(A, x(n))

        println("matcond(", n, ", ", c, ") & ", rank(A), " & ", round(cond(A); sigdigits = 4, base = 10), " & ", round(norm(first(A, my_b) - x(n)) / norm(x(n)); sigdigits = 4, base = 10), " & ", round(norm(second(A, my_b) - x(n))/ norm(x(n)); sigdigits = 4, base = 10), "\\\\")
    end
end