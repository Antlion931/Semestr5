module blocksys
    import ..matrixes_representation: MatrixOfCoeficients, MatrixOfCoeficientsWithPartialSelection, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get, last_meaningful_index_in_row, first_meaningful_index_in_row, RHM_from_vector

    export Axb, AxbWithPartialSelection, ALU, LUxb, ALUWithPartialSelection, LUxbWithPartialSelection

    function Axb(A::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        l = A.l
        for x in 1:(n-1)
            last_x = last_meaningful_index_in_row(A, x)
            for k in 1:(l-(x% l))
                y = x + k
                multiplayer = get(A, x, y)/get(A, x, x);
                for xx in (x+1):last_x
                    set(A, xx, y, get(A, xx, y) - multiplayer*get(A, xx, x))
                end
                set(b, 1, y, get(b, 1, y) - multiplayer*get(b, 1, x))
            end
        end

        return x_from_triangle_matrix_and_b(A, b)
    end

    function x_from_triangle_matrix_and_b(A::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        results = zeros(Float64, n)
        for y in n:-1:1
            if abs(get(A, y, y)) < eps(Float64)
                println("value too small, possible wrong results!")
            end
            sum = get(b, 1, y)
            for x in (y+1):last_meaningful_index_in_row(A, y)
                sum -= get(A, x, y)*results[x]
            end
            results[y] = sum/get(A, y, y)
        end
        
        return results
    end

    function AxbWithPartialSelection(A::MatrixOfCoeficientsWithPartialSelection, b::RightHandMatrix)
        n = length(b.body)
        l = A.l
        for x in 1:(n-1)
            last_row_in_block = ((x-1) ÷ l)*l + l

            max_row = x
            max = abs(get(A, x, x))
            for k in 1:(l-(x% l))
                y = x + k
                possible = abs(get(A, x, y))
                if possible > max
                    max_row = y
                    max = possible
                end
            end

            if max_row != x
                swap(A, x, max_row)
                swap(b, x, max_row)
            end

            last_x = last_meaningful_index_in_row(A, x)
            for k in 1:(l-(x% l))
                y = x + k
                multiplayer = get(A, x, y)/get(A, x, x);
                for xx in x:last_x
                    set(A, xx, y, get(A, xx, y) - multiplayer*get(A, xx, x))
                end
                set(b, 1, y, get(b, 1, y) - multiplayer*get(b, 1, x))
            end
        end

        return x_from_triangle_matrix_and_b_after_swaps(A, b)
    end

    function x_from_triangle_matrix_and_b_after_swaps(A::MatrixOfCoeficientsWithPartialSelection, b::RightHandMatrix)
        n = length(b.body)
        results = zeros(Float64, n)
        for y in n:-1:1
            if abs(get(A, y, y)) < eps(Float64)
                println("value too small, possible wrong results!")
            end
            
            sum = get(b, 1, y)
            for x in (y+1):last_meaningful_index_in_row(A, y)
                sum -= get(A, x, y)*results[x]
            end
            results[y] = sum/get(A, y, y)
        end
        
        for i in 1:n
            if A.swaped_indexes[i] != i
                results[i], results[A.swaped_indexes[i]] = results[A.swaped_indexes[i]], results[i]
                A.swaped_indexes[A.swaped_indexes[i]] = A.swaped_indexes[i]
           end
        end

        return results
    end

    function ALU(A::MatrixOfCoeficients)
        n = length(A.body)
        l = A.l
        for x in 1:(n-1)
            last_x = last_meaningful_index_in_row(A, x)
            for k in 1:(l-(x% l))
                y = x + k
                multiplayer = get(A, x, y)/get(A, x, x);
                set(A, x, y, multiplayer)
                for xx in (x+1):last_x
                    set(A, xx, y, get(A, xx, y) - multiplayer*get(A, xx, x))
                end
            end
        end

        return A
    end

    function LUxb(LU::MatrixOfCoeficients, b::RightHandMatrix)
        y = RHM_from_vector(x_from_L_matrix_and_b(LU, b))
        return x_from_triangle_matrix_and_b(LU, y)
    end

    function x_from_L_matrix_and_b(L::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        results = zeros(Float64, n)
        for y in 1:n
            sum = get(b, 1, y)
            for x in first_meaningful_index_in_row(L, y):(y-1)
                sum -= get(L, x, y)*results[x]
            end
            results[y] = sum
        end
        
        return results
    end

    function ALUWithPartialSelection(A::MatrixOfCoeficientsWithPartialSelection)
        n = length(A.body)
        l = A.l
        for x in 1:(n-1)
            last_row_in_block = ((x-1) ÷ l)*l + l

            max_row = x
            max = abs(get(A, x, x))
            for y in (x+1):last_row_in_block
                possible = abs(get(A, x, y))
                if possible > max
                    max_row = y
                    max = possible
                end
            end

            if max_row != x
                swap(A, x, max_row)
            end

            last_x = last_meaningful_index_in_row(A, x)
            for k in 1:(l-(x% l))
                y = x + k
                multiplayer = get(A, x, y)/get(A, x, x);
                set(A, x, y, multiplayer)
                for xx in (x+1):last_x
                    set(A, xx, y, get(A, xx, y) - multiplayer*get(A, xx, x))
                end
            end
        end

        return A
    end

    function LUxbWithPartialSelection(LU::MatrixOfCoeficientsWithPartialSelection, b::RightHandMatrix)
        y = RHM_from_vector(x_from_L_matrix_and_b_after_swaps(LU, b))
        return x_from_triangle_matrix_and_b_after_swaps(LU, y)
    end

    function x_from_L_matrix_and_b_after_swaps(L::MatrixOfCoeficientsWithPartialSelection, b::RightHandMatrix)
        n = length(b.body)
        results = zeros(Float64, n)
        for y in 1:n
            sum = get(b, 1, y)
            for x in first_meaningful_index_in_row(L, y):(y-1)
                sum -= get(L, x, y)*results[x]
            end
            results[y] = sum
        end

        for i in 1:n
            if L.swaped_indexes[i] != i
                results[i], results[L.swaped_indexes[i]] = results[L.swaped_indexes[i]], results[i]
                L.swaped_indexes[L.swaped_indexes[i]] = L.swaped_indexes[i]
            end
        end
        
        return results
    end
end
