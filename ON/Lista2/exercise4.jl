# Albert Kołodziejski
import Pkg;
Pkg.add("Polynomials")
using Polynomials

a=reverse([1, -210.0, 20615.0,-1256850.0,
      53327946.0,-1672280820.0, 40171771630.0, -756111184500.0,          
      11310276995381.0, -135585182899530.0,
      1307535010540395.0,     -10142299865511450.0,
      63030812099294896.0,     -311333643161390640.0,
      1206647803780373360.0,     -3599979517947607200.0,
      8037811822645051776.0,      -12870931245150988800.0,
      13803759753640704000.0,      -8752948036761600000.0,
      2432902008176640000.0])

disrupted_a=reverse([1, -210.0 - 2^-23, 20615.0,-1256850.0,
53327946.0,-1672280820.0, 40171771630.0, -756111184500.0,          
11310276995381.0, -135585182899530.0,
1307535010540395.0,     -10142299865511450.0,
63030812099294896.0,     -311333643161390640.0,
1206647803780373360.0,     -3599979517947607200.0,
8037811822645051776.0,      -12870931245150988800.0,
13803759753640704000.0,      -8752948036761600000.0,
2432902008176640000.0])

function p(x)
    (x - 20.0)*(x - 19.0)*(x - 18.0)*(x - 17.0)*(x - 16.0)*
    (x - 15.0)*(x - 14.0)*(x - 13.0)*(x - 12.0)*(x - 11.0)*
    (x - 10.0)*(x - 9.0)*(x - 8.0)*(x - 7.0)*(x - 6.0)*
    (x - 5.0)*(x - 4.0)*(x - 3.0)*(x - 2.0)*(x - 1.0)
end

poly = Polynomial(a)

z = roots(poly)

println("k & z & |P(z)| & |p(z)| & |z - k|")
for k = 1:20
    println(k, " & ", z[k], " & ", round(abs(poly(z[k])); sigdigits = 4, base = 10), " & ", round(abs(p(z[k])); sigdigits = 4, base = 10), " & ", round(abs(z[k] - k); sigdigits = 4, base = 10), "\\\\")
end

println("roots of distrupted")

poly = Polynomial(a)

z = roots(Polynomial(disrupted_a))

println("k & z & |P(z)| & |p(z)| & |z - k|")
for k = 1:20
    println(k, " & ", z[k], " & ", round(abs(poly(z[k])); sigdigits = 4, base = 10), " & ", round(abs(p(z[k])); sigdigits = 4, base = 10), " & ", round(abs(z[k] - k); sigdigits = 4, base = 10), "\\\\")
end