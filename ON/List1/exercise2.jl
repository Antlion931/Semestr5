# Albert Kołodziejski

function experiment(T)
    T(3) * ( T(4) / T(3)  - T(1)) - T(1)
end

println("experiment Float16: ", experiment(Float16))
println("eps Float16: ", eps(Float16))

println("experiment Float32: ", experiment(Float32))
println("eps Float32: ", eps(Float32))

println("experiment Float64: ", experiment(Float64))
println("eps Float64: ", eps(Float64))