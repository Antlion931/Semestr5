# Albert Kołodziejski

module matrixes_representation

    export MatrixOfCoeficients, new_MOC

    struct MatrixOfCoeficients
        l::Int64
        is_partial_selection::Bool
        body::Vector{Vector{Float64}}
    end

    function new_MOC(l::Int64, n::Int64, is_partial_selection::Bool)
        body = Vector{Vector{Float64}}()
        sizehint!(body, n ÷ l)
        
        if l == n
            for i in 1:n
                push!(body, zeros(Float64, n))
            end
        else if is_partial_selection
            for i in 1:l
                push!(body, zeros(Float64, 2 * l))
            end

            for i in 3:(n ÷ l)
                for k in 1:l
                    push!(body, zeros(Float64, 1 + 2 * l))
                end
            end

            for i in 1:l
                push!(body, zeros(Float64, 1 + l))
            end
        else
            for i in 1:l
                push!(body, zeros(Float64, l + i))
            end

            for i in 3:(n ÷ l)
                for k in 1:l
                    push!(body, zeros(Float64, 1 + l + k))
                end
            end

            for i in 1:l
                push!(body, zeros(Float64, 1 + l))
            end
        end

        return MatrixOfCoeficients(l, is_partial_selection, body)
    end
end