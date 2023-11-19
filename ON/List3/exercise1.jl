#Albert Kołodziejski

function mbisekcji(f, a::Float64, b::Float64, delta::Float64, epsilon::Float64)
    iteration = 0
    u = f(a)

    if abs(u) < epsilon 
        return (a, u, iteration, 0)
    end
    v = f(b)

    if abs(v) < epsilon 
        return (b, v, iteration, 0)
    end

    e = (b - a)/2
    iteration += 1
    c = a + e
    w = f(c)

    # error, function doesn't change, if zero cases covered
    if sign(u) == sign(v)
        return (a, u, iteration, 1)
    end

    while abs(e) >= delta && abs(w) >= epsilon
        if sign(w) != sign(u)
            b = c
            v = w
        else
            a = c
            u = w
        end

        e = e/2
        iteration += 1
        c = a + e
        w = f(c)
    end

    return (c, w, iteration, 0)
end

