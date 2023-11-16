# Albert Kołodziejski

function iteration(last, r, T)
    T(last) + T(r) * T(last)*(T(1) - T(last))
end

r = 3

println("Float32")
p = [Float32(0.01)]
for i = 1:40 
    push!(p, iteration(p[end], r, Float32))
end
println(p)

println("disrupted Float32")
p = [Float32(0.01)]
for i = 1:40 
    if i == 10
        new = trunc(iteration(p[end], r, Float32), digits=3)
        push!(p, new)
    else
        push!(p, iteration(p[end], r, Float32))
    end
end
println(p)


println("Float64")
p = [Float64(0.01)]
for i = 1:40 
    push!(p, iteration(p[end], r, Float64))
end
println(p)
