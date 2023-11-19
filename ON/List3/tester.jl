#Albert Kołodziejski

include("ZeroPointModule.jl")
using ArgParse

using Printf

function parse_commandline()
    s = ArgParseSettings()

    @add_arg_table! s begin
        "--bisection"
            help = "bisection method"
            action = :store_true
        "--newton"
            help = "newton's method"
            action = :store_true
        "--secant"
            help = "secant method"
            action = :store_true
        "-f"
            help = "function"
        "--pf"
            help = "derivitive of function"
        "--x0"
            help = "x0"
            arg_type = Float64
        "--x1"
            help = "x1"
            arg_type = Float64
        "-d"
            help = "delta"
            arg_type = Float64
        "-e"
            help = "epsilon"
            arg_type = Float64
        "-m"
            help = "max iteration"
            arg_type = Int
    end

    return parse_args(s)
end

function main()
    parsed_args = parse_commandline()

    if parsed_args["bisection"]
        function f1(number)
            r = replace(parsed_args["f"], " x " => @sprintf(" reinterpret(Float64, UInt64(%d))", reinterpret(UInt64, number)))
            return eval(Meta.parse(r))
        end

        println(ZeroPointModule.mbisekcji(f1, parsed_args["x0"], parsed_args["x1"], parsed_args["d"], parsed_args["e"]))
    end

    if parsed_args["newton"]
        function f2(number)
            r = replace(parsed_args["f"], " x " => @sprintf(" reinterpret(Float64, UInt64(%d))", reinterpret(UInt64, number)))
            return eval(Meta.parse(r))
        end

        function pf(number)
            r = replace(parsed_args["pf"], " x " => @sprintf(" reinterpret(Float64, UInt64(%d))", reinterpret(UInt64, number)))
            return eval(Meta.parse(r))
        end

        println(ZeroPointModule.mstycznych(f2, pf, parsed_args["x0"], parsed_args["d"], parsed_args["e"], parsed_args["m"]))
    end

    if parsed_args["secant"]
        function f3(number)
            r = replace(parsed_args["f"], " x " => @sprintf(" reinterpret(Float64, UInt64(%d))", reinterpret(UInt64, number)))
            return eval(Meta.parse(r))
        end

        println(ZeroPointModule.msiecznych(f3, parsed_args["x0"], parsed_args["x1"], parsed_args["d"], parsed_args["e"], parsed_args["m"]))
    end
end

main()