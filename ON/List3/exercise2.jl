#Albert Kołodziejski

function mstycznych(f,pf,x0::Float64, delta::Float64, epsilon::Float64, maxit::Int)
    v = f(x0)
    if abs(v) < epsilon
        return (x0, v, 0, 0)
    end

    for k in 1:maxit
        u = v/pf(x0) 

        if isinf(u) || isnan(u) 
            return (x0, v, k, 2)
        end

        x1 = x0 - u
        v = f(x1)

        if abs(x1 - x0) < delta || abs(v) < epsilon
            return (x1, v, k, 0)
        end
        x0 = x1
    end

    return (x0, v, maxit + 1, 1)
end