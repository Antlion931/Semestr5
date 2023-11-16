# Albert Kołodziejski

function iteration(last, c)
    Float64(last)^2 + Float64(c)
end

function experiment(c, x)
    println("c = ", c, ", x = ", x)
    p = [Float64(x)]
    for i = 1:40 
        push!(p, iteration(p[end], c))
    end
    println(p)

end

experiment(-2, 1)
experiment(-2, 2)
experiment(-2, 1.99999999999999)
experiment(-1, 1)
experiment(-1, -1)
experiment(-1, 0.75)
experiment(-1, 0.25)
