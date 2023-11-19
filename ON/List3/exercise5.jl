#Albert Kołodziejski

include("ZeroPointModule.jl")

f = x -> ℯ^x - 3*x
delta = 10^-4
epsilon = 10^-4


println("1: ", ZeroPointModule.mbisekcji(f, 0.0, 1.0, delta, epsilon))

println("2: ", ZeroPointModule.mbisekcji(f, 1.0, 2.0, delta, epsilon))