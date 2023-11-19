#Albert Kołodziejski

include("ZeroPointModule.jl")

f1 = x -> ℯ^(1 - x) - 1
pf1 = x -> -ℯ^(1 - x)
f2 = x -> x*ℯ^(-x)
pf2 = x -> -ℯ^(-x) * (x - 1)

delta = 10^-5
epsilon = 10^-5

println("bisekcji 1: ", ZeroPointModule.mbisekcji(f1, -3.14, 1.618, delta, epsilon))
println("bisekcji 2: ", ZeroPointModule.mbisekcji(f2, -3.14, 1.618, delta, epsilon))

println("stycznych (f1, pf1, -1.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, -1.0, delta, epsilon, 100))
println("stycznych (f2, pf2, -1.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f2, pf2, -1.0, delta, epsilon, 100))

println("siecznych 1: ", ZeroPointModule.msiecznych(f1, -1.0, -2.0, delta, epsilon, 100))
println("siecznych 2: ", ZeroPointModule.msiecznych(f2, -1.0, -2.0, delta, epsilon, 100))

println("stycznych (f1, pf1, 2.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 2.0, delta, epsilon, 100))
println("stycznych (f1, pf1, 3.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 3.0, delta, epsilon, 100))
println("stycznych (f1, pf1, 4.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 4.0, delta, epsilon, 100))
println("stycznych (f1, pf1, 5.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 5.0, delta, epsilon, 100))
println("stycznych (f1, pf1, 6.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 6.0, delta, epsilon, 1000))
println("stycznych (f1, pf1, 7.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 7.0, delta, epsilon, 1000))
println("stycznych (f1, pf1, 8.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 8.0, delta, epsilon, 100))
println("stycznych (f1, pf1, 9.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f1, pf1, 9.0, delta, epsilon, 100))
println("stycznych (f2, pf2, 1.1, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f2, pf2, 1.1, delta, epsilon, 100))
println("stycznych (f2, pf2, 1.01, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f2, pf2, 1.01, delta, epsilon, 100))
println("stycznych (f2, pf2, 1.001, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f2, pf2, 1.001, delta, epsilon, 100))
println("stycznych (f2, pf2, 1.0, delta, epsilon, 100): ", ZeroPointModule.mstycznych(f2, pf2, 1.0, delta, epsilon, 100))
