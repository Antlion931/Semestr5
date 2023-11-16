# Albert Kołodziejski

# for i = 37 test = 100 because e^-x is too small and 1 eats it, so there is zero in formula

function test(x)
    ℯ^-x + 1
end

for i = 1:40
    println( i, ",", test(i))
end