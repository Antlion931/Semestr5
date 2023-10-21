x = [2.718281828, -3.141592654, 1.414213562, 0.5772156649, 0.3010299957]
y = [1486.2497, 878366.9879, -22.37492, 4773714.647, 0.000185049]

correct_result = Float64(-1.00657107000000 * 10^-11)

function my_print(text, value)
    e = abs(Float64(value) - correct_result)
    println(text, value, " & ", e, " & ", e/abs(correct_result))
end

function scalar_forward(x, y, T)
    sum = T(0)
    for pair in zip(x, y)
        sum += T(pair[1]) * T(pair[2])
    end
    sum
end

function scalar_backward(x, y, T)
    scalar_forward(reverse(x), reverse(y), T)
end

function sum(array, T)
    sum = T(0)
    for a in array
        sum += T(a)
    end
    sum
end

function smaller_c_to_bigger(x, y, T, rev)
    positive = []
    negative = []

    for pair in zip(x, y)
        a = T(pair[1]) * T(pair[2])
        if a >= 0 
            push!(positive, a)
        else
            push!(negative, a)
        end
    end

    if rev 
        sort(positive, rev=true)
        sort(negative, rev=false)
    else 
        sort(positive, rev=false)
        sort(negative, rev=true)
    end

    sum(positive, T) + sum(negative, T)
end

my_print("scalar forward Float32: ", scalar_forward(x, y, Float32))
my_print("scalar backward Float32: ", scalar_backward(x, y, Float32))
my_print("scalar ascending Float32: ", smaller_c_to_bigger(x, y, Float32, false))
my_print("scalar descending Float32: ", smaller_c_to_bigger(x, y, Float32, true))

println()

my_print("scalar forward Float64: ", scalar_forward(x, y, Float64))
my_print("scalar backward Float64: ", scalar_backward(x, y, Float64))
my_print("scalar ascending Float64: ", smaller_c_to_bigger(x, y, Float64, false))
my_print("scalar descending Float64: ", smaller_c_to_bigger(x, y, Float64, true))
