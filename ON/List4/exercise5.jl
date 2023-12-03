#Albert Kołodziejski

include("exercise4.jl")

rysujNnfx(x -> ℯ^x, 0.0, 1.0, 5)
rysujNnfx(x -> ℯ^x, 0.0, 1.0, 10)
rysujNnfx(x -> ℯ^x, 0.0, 1.0, 15)

rysujNnfx(x -> x^2.0 * sin(x), -1.0, 1.0, 5)
rysujNnfx(x -> x^2.0 * sin(x), -1.0, 1.0, 10)
rysujNnfx(x -> x^2.0 * sin(x), -1.0, 1.0, 15)