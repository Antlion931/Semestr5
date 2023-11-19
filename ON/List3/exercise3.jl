#Albert Kołodziejski

function msiecznych(f, x0::Float64, x1::Float64, delta::Float64, epsilon::Float64,maxit::Int)
    f0 = f(x0)
    f1 = f(x1)

    for k in 1:maxit
        if abs(f0) > abs(f1)
            x0, x1 = x1, x0
            f0, f1 = f1, f0
        end

        s = (x1 - x0)/(f1 - f0)
        x1 = x0
        f1 = f0
        x0 = x0 - f0*s
        f0 = f(x0)

        if abs(x1 - x0) < delta || abs(f0) < epsilon
            return (x0, f0, k, 0)
        end
    end

    return (x0, f0, maxit + 1, 1)
end