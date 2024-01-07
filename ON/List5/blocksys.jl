module blocksys
    import ..matrixes_representation: MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get, last_index_in_row

    export Axb

    function Axb(A::MatrixOfCoeficients, b::RightHandMatrix)
        n = length(b.body)
        l = A.l
        for x in 1:(n-1)
            for k in 1:(l-(x%4))
                y = x + k
                multiplayer = -get(A, x, y)/get(A, y, y);
                println("multiplayer: ", multiplayer, " end: ", last_index_in_row(A, y))
            end
        end
    end
end
