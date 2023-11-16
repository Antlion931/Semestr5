# Albert Kołodziejski

function experiment()
    x = Float64(1)
    step = Float64(2) ^ -52

    while x < 2 && x*(1/x) == 1
        x += step
    end

    x
end

println(experiment())