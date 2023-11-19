#Albert Kołodziejski

include("ZeroPointModule.jl")

f = x -> sin(x) - (x/2)^2
pf = x -> cos(x) - x/2
delta = (1/2) * 10^-5
epsilon = (1/2) * 10^-5

println("bisekcji: ", ZeroPointModule.mbisekcji(f, 1.5, 2.0, delta, epsilon))
println("stycznych: ", ZeroPointModule.mstycznych(f, pf, 1.5, delta, epsilon, 100))
println("siecznych: ", ZeroPointModule.msiecznych(f, 1.0, 2.0, delta, epsilon, 100))