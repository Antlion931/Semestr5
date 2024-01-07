# Albert Kołodziejski

module matrixes_representation

    export MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get

    # Define an abstract type for the common interface
    abstract type MatrixInterface end

    function set(obj::MatrixInterface, x::Int64, y::Int64, value::Float64)
        error("set method not implemented for $(typeof(obj))")
    end

    function swap(obj::MatrixInterface, y1::Int64, y2::Int64)
        error("swap method not implemented for $(typeof(obj))")
    end

    function get(obj::MatrixInterface, x::Int64, y::Int64)
        error("get method not implemented for $(typeof(obj))")
    end

    struct MatrixOfCoeficients <: MatrixInterface
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
        elseif is_partial_selection
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
        if y > obj.l
            x += 1
        end

        return x - ((y-1) ÷ obj.l)*obj.l
    end

    function get(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        x = local_x(obj, x, y)
        return obj.body[y][x]
    end

    function set(obj::MatrixOfCoeficients, x::Int64, y::Int64, value::Float64)
        x = local_x(obj, x, y)
        obj.body[y][x] = value
    end

    function swap(obj::MatrixOfCoeficients, y1::Int64, y2::Int64)
        obj.body[y1], obj.body[y2] = obj.body[y2], obj.body[y1]
    end

    function compute_b_with_x_of_ones(obj::MatrixOfCoeficients)
        n = length(obj.body)
        b = new_RHM(n)
        for i in 1:n
            set(b, 1, i, sum(obj.body[i]))
        end
        return b
    end

    struct RightHandMatrix <: MatrixInterface
        body::Vector{Float64}
    end

    function new_RHM(n::Int64)
        RightHandMatrix(zeros(Float64, n))
    end

    function get(obj::RightHandMatrix, x::Int64, y::Int64)
        return obj.body[y]
    end

    function set(obj::RightHandMatrix, x::Int64, y::Int64, value::Float64)
        obj.body[y] = value
    end

    function swap(obj::RightHandMatrix, y1::Int64, y2::Int64)
        obj.body[y1], obj.body[y2] = obj.body[y2], obj.body[y1]
    end
end