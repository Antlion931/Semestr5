function integral_approximation(f, x, h)
    (f(x + h) - f(x)) / h
end

function f(x)
    sin(x) + cos(x)
end

correct_integral = Float64(0.1169422816885380510987021990186457641915106278611254752144493)

function experiment(plus_to_h)
    for n = 0:54
        h = Float64(2)^-n
        approx = integral_approximation(f, Float64(1), h + plus_to_h)
        println("n = ", n)
        println("integral approximation = ", approx)
        println("abs difference = ", abs(correct_integral - approx))
    end
end

println("h")

experiment(0.0)

println("1 + h")

experiment(1.0)
