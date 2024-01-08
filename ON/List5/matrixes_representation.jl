# Albert Kołodziejski

module matrixes_representation

    export MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get, last_meaningful_index_in_row, first_meaningful_index_in_row, MatrixOfCoeficientsWithPartialSelection, new_MOCWPS, swap, RHM_from_vector

    # Define an abstract type for the common interface
    abstract type MatrixInterface end

    function set(obj::MatrixInterface, x::Int64, y::Int64, value::Float64)
        error("set method not implemented for $(typeof(obj))")
    end

    function get(obj::MatrixInterface, x::Int64, y::Int64)
        error("get method not implemented for $(typeof(obj))")
    end

    struct MatrixOfCoeficients <: MatrixInterface
        l::Int64
        body::Vector{Vector{Float64}}
    end

    function new_MOC(l::Int64, n::Int64)
        body = Vector{Vector{Float64}}()
        sizehint!(body, n ÷ l)
        
        if l == n
            for i in 1:n
                push!(body, zeros(Float64, n))
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

        return MatrixOfCoeficients(l, body)
    end

    function local_x(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        if y > obj.l
            x += 1
        end

        return x - ((y-1) ÷ obj.l)*obj.l
    end

    function global_x(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        if y > obj.l
            x -= 1
        end

        return x + ((y-1) ÷ obj.l)*obj.l
    end

    function get(obj::MatrixOfCoeficients, x::Int64, y::Int64)
        x = local_x(obj, x, y)
        return obj.body[y][x]
    end

    function set(obj::MatrixOfCoeficients, x::Int64, y::Int64, value::Float64)
        x = local_x(obj, x, y)
        obj.body[y][x] = value
    end

    function last_meaningful_index_in_row(obj::MatrixOfCoeficients, y::Int64)
        global_x(obj, length(obj.body[y]), y)
    end

    function first_meaningful_index_in_row(obj::MatrixOfCoeficients, y::Int64)
        global_x(obj, 1, y)
    end

    function compute_b_with_x_of_ones(obj::MatrixOfCoeficients)
        n = length(obj.body)
        b = new_RHM(n)
        for i in 1:n
            set(b, 1, i, sum(obj.body[i]))
        end
        return b
    end

    struct MatrixOfCoeficientsWithPartialSelection <: MatrixInterface
        l::Int64
        swaped_indexes::Vector{Int64}
        last_meaningful_indexes::Vector{Int64}
        body::Vector{Vector{Float64}}
    end

    function new_MOCWPS(l::Int64, n::Int64)
        body = Vector{Vector{Float64}}()
        sizehint!(body, n ÷ l)
        swaped_indexes = collect(1:n)
        last_meaningful_indexes = Vector{Int64}()
        sizehint!(last_meaningful_indexes, n)

        if l == n
            for i in 1:n
                push!(last_meaningful_indexes, n)
                push!(body, zeros(Float64, n))
            end
        else
            for i in 1:l
                push!(last_meaningful_indexes, l + i)
                push!(body, zeros(Float64, 2 * l))
            end

            for i in 3:(n ÷ l)
                for k in 1:l
                    push!(last_meaningful_indexes, 1 + l + k)
                    push!(body, zeros(Float64, 1 + 2 * l))
                end
            end

            for i in 1:l
                push!(last_meaningful_indexes, 1 + l)
                push!(body, zeros(Float64, 1 + l))
            end
        end

        return MatrixOfCoeficientsWithPartialSelection(l, swaped_indexes, last_meaningful_indexes, body)
    end

    function local_x(obj::MatrixOfCoeficientsWithPartialSelection, x::Int64, y::Int64)
        if y > obj.l
            x += 1
        end

        return x - ((y-1) ÷ obj.l)*obj.l
    end

    function global_x(obj::MatrixOfCoeficientsWithPartialSelection, x::Int64, y::Int64)
        if y > obj.l
            x -= 1
        end

        return x + ((y-1) ÷ obj.l)*obj.l
    end

    function get(obj::MatrixOfCoeficientsWithPartialSelection, x::Int64, y::Int64)
        yy = obj.swaped_indexes[y]
        xx = local_x(obj, x, yy)
        
        return obj.body[yy][xx]
    end

    function set(obj::MatrixOfCoeficientsWithPartialSelection, x::Int64, y::Int64, value::Float64)
        y = obj.swaped_indexes[y]
        x = local_x(obj, x, y)
        if x > obj.last_meaningful_indexes[y]
            obj.last_meaningful_indexes[y] = x
        end
        obj.body[y][x] = value
    end

    function swap(obj::MatrixOfCoeficientsWithPartialSelection, y1::Int64, y2::Int64)
        obj.swaped_indexes[y1], obj.swaped_indexes[y2] = obj.swaped_indexes[y2], obj.swaped_indexes[y1]
    end

    function last_meaningful_index_in_row(obj::MatrixOfCoeficientsWithPartialSelection, y::Int64)
        y = obj.swaped_indexes[y]
        global_x(obj, obj.last_meaningful_indexes[y], y)
    end

    function first_meaningful_index_in_row(obj::MatrixOfCoeficientsWithPartialSelection, y::Int64)
        y = obj.swaped_indexes[y]
        global_x(obj, 1, y)
    end

    function compute_b_with_x_of_ones(obj::MatrixOfCoeficientsWithPartialSelection)
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

    function RHM_from_vector(v::Vector{Float64})
        return RightHandMatrix(v)
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
