include("matrixes_representation.jl")
include("blocksys.jl")

using ArgParse
using .blocksys: Axb
using .matrixes_representation: MatrixOfCoeficients, new_MOC, RightHandMatrix, new_RHM, compute_b_with_x_of_ones, MatrixInterface, set, swap, get

function parse_commandline()
    s = ArgParseSettings()

    @add_arg_table! s begin
        "--A"
            help = "File with matrix A"
            arg_type = String
            required = true
        "--b"
            help = "File with matrix b, if not given, b is generated based on x = [1, 1, ..., 1]"
            arg_type = String
        "--selection"
            help = "set partial selection on"
            action = :store_true
        "--save"
            help = "File to save the result"
            arg_type = String
        "--action"
            help = "Specifies the action to do with data, Axb for Ax = b, LUxb for LUx = b"
            arg_type = String
            required = true
    end

    return parse_args(s)
end

function read_MOC(filename::String)
    file = open(filename, "r")
    n, l = split(readline(file), ' ')
    n = parse(Int64, n)
    l = parse(Int64, l)
    moc = new_MOC(l, n)
    for line in eachline(file)
        y, x, value = split(line, ' ')
        set(moc, parse(Int64, x), parse(Int64, y), parse(Float64, value))
    end
    close(file)
    return moc
end

function read_MOCWPS(filename::String)
    file = open(filename, "r")
    n, l = split(readline(file), ' ')
    n = parse(Int64, n)
    l = parse(Int64, l)
    mocwps = new_MOCWPS(l, n)
    for line in eachline(file)
        y, x, value = split(line, ' ')
        set(mocwps, parse(Int64, x), parse(Int64, y), parse(Float64, value))
    end
    close(file)
    return moc
end

function read_RHM(filename::String)
    file = open(filename, "r")
    n = parse(Int64, readline(file))
    rhm = new_RHM(n)
    
    for y in 1:n
        set(rhm, 1, y, parse(Float64, readline(file)))
    end

    close(file)
    return rhm
end

function main()
    parsed_args = parse_commandline()

    A = nothing
    
    if parsed_args["selection"]
        A = read_MOCWPS(parsed_args["A"])
    else
        A = read_MOC(parsed_args["A"])
    end

    b = nothing

    if isnothing(parsed_args["b"])
        b = compute_b_with_x_of_ones(A)
    else
        b = read_RHM(parsed_args["b"])
    end

    if parsed_args["action"] == "Axb"
        Axb(A, b)
    end
end

main()