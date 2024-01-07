module blocksys
    import ..matrixes_representation: MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get, last_meaningful_index_in_row

    export Axb

    function Axb(A::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        l = A.l
        for x in 1:(n-1)
            last_x = last_meaningful_index_in_row(A, x)
            for k in 1:(l-(x%4))
                y = x + k
                multiplayer = -get(A, x, y)/get(A, x, x);
                for xx in x:last_x
                    set(A, xx, y, get(A, xx, y) + multiplayer*get(A, xx, x))
                end
                set(b, 1, y, get(b, 1, y) + multiplayer*get(b, 1, x))
            end
        end

        return x_from_triangle_matrix_and_b(A, b)
    end

    function x_from_triangle_matrix_and_b(A::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        results = zeros(Float64, n)
        for y in n:-1:1
            sum = get(b, 1, y)
            for x in (y+1):last_meaningful_index_in_row(A, y)
                sum -= get(A, x, y)*results[x]
            end
            results[y] = sum/get(A, y, y)
        end
        
        return results
    end
end
