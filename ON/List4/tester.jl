#Albert Kołodziejski

include("PolynomialsModule.jl")

println(PolynomialsModule.ilorazyRoznicowe([3.0, 1.0, 5.0, 6.0], [1.0, -3.0, 2.0, 4.0]))
println(PolynomialsModule.warNewton([1.0, 2.0, 3.0], PolynomialsModule.ilorazyRoznicowe([1.0, 2.0, 3.0], [1.0, 4.0, 9.0]), 3.2))
println(PolynomialsModule.naturalna([1.0, 2.0, 3.0], PolynomialsModule.ilorazyRoznicowe([1.0, 2.0, 3.0], [1.0, 4.0, 9.0])))