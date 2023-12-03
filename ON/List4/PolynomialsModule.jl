#Albert Kołodziejski

module PolynomialsModule

export ilorazyRoznicowe, warNewton, naturalna

function ilorazyRoznicowe(x::Vector{Float64}, f::Vector{Float64})
    length = size(x)[1]
    result = copy(f)

    for i in 1:length-1
        for k in length:-1:i+1
            if x[k] == x[k - i]
                error("Two identical x values")
            end
            
            result[k] = (result[k] - result[k - 1]) / (x[k] - x[k - i])
        end
    end

    return result
end


function warNewton(x::Vector{Float64}, fx::Vector{Float64}, t::Float64)
    w = fx[end]

    for i in 1:size(x)[1] - 1
        w = fx[end - i] + (t - x[end - i]) * w
    end

    return w
end


function naturalna(x::Vector{Float64}, fx::Vector{Float64})

    w = Vector{Float64}()

    push!(w, fx[end])

    for i in size(x)[1]-1:-1:1
        a = w * x[i]
        push!(a, 0.0)

        pushfirst!(w, fx[i])

        w = w - a
    end

    return w
end
end