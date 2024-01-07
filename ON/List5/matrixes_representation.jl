# Albert Kołodziejski

module matrixes_representation

    export MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, get, set, swap

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

    function local_x(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        return (y ÷ obj.l)*obj.l - x
    end

    function get(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        x = obj.local_x(x, y)
        return obj.body[x][y]
    end

    function set(obj::MatrixOfCoeficients, x::Int64, y::Int64, value::Float64)
        x = obj.local_x(x, y)
        obj.body[x][y] = value
    end

    function swap(obj::MatrixOfCoeficients, y1::Int64, y2::Int64)
        obj.body[y1], obj.body[y2] = obj.body[y2], obj.body[y1]
    end


    struct RightHandMatrix
        body::Vector{Float64}
    end

    function new_RHM(n::Int64)
        RightHandMatrix(zeros(Float64, n))
    end

    function get(obj::RightHandMatrix, y::Int64)
        return obj.body[y]
    end

    function set(obj::RightHandMatrix, y::Int64, value::Float64)
        obj.body[y] = value
    end

    function swap(obj::RightHandMatrix, y1::Int64, y2::Int64)
        obj.body[y1], obj.body[y2] = obj.body[y2], obj.body[y1]
    end
end