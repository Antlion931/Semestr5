x = Float64(1)
step = Float64(2) ^ -52

println("[1, 2]")

println(bitstring(x))
println(bitstring(x + step))
println(bitstring(x + 2*step))
println(bitstring(x + 3*step))
println(bitstring(x + (2^52 - 2)*step))
println(bitstring(x + (2^52 - 1)*step))
println(bitstring(x + (2^52)*step))

println("[2, 4]")

x = Float64(2)
step = Float64(2) ^ -51

println(bitstring(x))
println(bitstring(x + step))
println(bitstring(x + 2*step))
println(bitstring(x + 3*step))
println(bitstring(x + (2^51 - 2)*step))
println(bitstring(x + (2^51 - 1)*step))
println(bitstring(x + (2^51)*step))

println("[0.5, 1]")

x = Float64(0.5)
step = Float64(2) ^ -53

println(bitstring(x))
println(bitstring(x + step))
println(bitstring(x + 2*step))
println(bitstring(x + 3*step))
println(bitstring(x + (2^51 - 2)*step))
println(bitstring(x + (2^51 - 1)*step))
println(bitstring(x + (2^51)*step))