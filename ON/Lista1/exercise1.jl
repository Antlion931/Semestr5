# Albert Kołodziejski

function my_eps(T)
    one = T(1)
    x = T(1)
    while one + x / T(2) > T(1)
        x /= T(2)
    end
    x
end

println("eps Float16: ", eps(Float16))
println("my eps Float16: ", my_eps(Float16))

println("eps Float32: ", eps(Float32))
println("my eps Float32: ", my_eps(Float32))

println("eps Float64: ", eps(Float64))
println("my eps Float64: ", my_eps(Float64))

function my_eta(T)
    x = T(1)
    while x/T(2) > 0
        x /= T(2)
    end
    x
end

println("nextfloat Float16: ", nextfloat(Float16(0)))
println("my eta Float16: ", my_eta(Float16))

println("nextfloat32: ", nextfloat(Float32(0)))
println("my eta Float32: ", my_eta(Float32))

println("nextfloat Float64: ", nextfloat(Float64(0)))
println("my eta Float64: ", my_eta(Float64))

function my_max(T)
    x = T(2) - my_eps(T) 
    while x * T(2)< Inf
        x *= T(2)
    end
    x 
end

println("floatmax Float16: ", floatmax(Float16))
println("my max Float16: ", my_max(Float16))

println("floatmax Float32: ", floatmax(Float32))
println("my max Float32: ", my_max(Float32))

println("floatmax Float64: ", floatmax(Float64))
println("my max Float64: ", my_max(Float64))