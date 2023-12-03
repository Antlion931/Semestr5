#Albert Kołodziejski

include("PolynomialsModule.jl")

using Pkg
Pkg.add("Plots")

using Plots

function points(f, a, b, step)
    x = a:step:b
    y = f.(x)

    return (x, y)
end

function rysujNnfx(f,a::Float64,b::Float64,n::Int)
    h = (b - a)/n
    xn = Vector{Float64}()
    vec_f = Vector{Float64}()

    for k in 0:n 
        push!(xn, a + k*h)
        push!(vec_f, f(a + k*h))
    end

    ir = PolynomialsModule.ilorazyRoznicowe(xn, vec_f)

    (x, y1) = points(x -> PolynomialsModule.warNewton(xn, ir, x), a, b, (b-a)/1000)
    (x, y2) = points(x -> f(x), a, b, (b-a)/1000)

    plot(x, [y1 y2], label=["wielomian interpolacyjny" "funkcja interpolowana"], title="n = $n", lw = [4 3])

    png("$f $n")
end
