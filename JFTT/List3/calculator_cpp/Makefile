FF := flex
FFLAGS :=
BB := bison
BFLAGS :=
CXX := g++
CXXFLAGS := -Wall -Wextra -pedantic -std=c++17

all: flex bison
	$(CXX) $(CXXFLAGS) -o calc calc_l.cpp calc_y.cpp gf.cpp

flex:
	$(FF) $(FFLAGS) -o calc_l.cpp calc.lex

bison:
	$(BB) $(BFLAGS) -d -o calc_y.cpp calc.y

clean:
	rm -f calc_lexer.cpp calc_parser.hpp calc_parser.cpp calc
