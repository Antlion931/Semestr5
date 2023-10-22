function f(x)
    x = Float64(x)
    sqrt(x^2 + 1) - 1
end

function g(x)
    x = Float64(x)
    x^2 / (sqrt(x^2 + 1) + 1)
end

for i = 0:179
    x = Float64(8)^-i
    println("x = 8^-", i)
    println("f(x) = ", f(x))
    println("g(x) = ", g(x))
end
