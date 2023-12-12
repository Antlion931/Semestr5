%{
#include <iostream>
#include <string>
#include <vector>
#include <sstream>
#include <optional>
#include <string>

#define YYSTYPE int64_t
#define P 1234577
#define Q 1234576

std::string rpn;
std::string exponent_rpn;
bool semantic_error;

#include "gf.hpp"
int yylex();
void yyerror(const char* s);
%}
%token PLUS
%token MINUS
%token MUL
%token DIV
%token POW
%token LEFT_P
%token RIGHT_P
%token VAL
%token ERROR
%token EOL

%%
input: %empty | input line;

line: as EOL {
        std::cout << "RPN: " << rpn << std::endl;
        if (semantic_error) {
            std::cout << "= error" << std::endl;
        } else {
            std::cout << "= " << $1 << std::endl;
        }

        semantic_error = false;
        rpn.clear();
        exponent_rpn.clear();
    }
    | ERROR EOL {
        std::cout << "Syntax error" << std::endl;
        semantic_error = false;
        rpn.clear();
        exponent_rpn.clear();
    }
    | EOL;

as: as PLUS md { $$ = add($1, $3, P); rpn += "+ ";}
  | as MINUS md { $$ = sub($1, $3, P); rpn += "- ";}
  | md { $$ = $1; };

md: md MUL n { $$ = mul($1, $3, P); rpn += "* ";}
  | md DIV n { 
   rpn += "/ ";
   auto r = div($1, $3, P);
       if (r.has_value()) {
           $$ = r.value();
       } else {
            semantic_error = true;
       }
   }
  | n { $$ = $1; };

n: MINUS n { $$ = neg($2, P); rpn += "~ "; }
 | e { $$ = $1; };

e: term POW exponent_n { $$ = pow($1, $3, P); rpn += exponent_rpn + "^ "; exponent_rpn.clear(); }
 | term { $$ = $1;};

term: VAL { $$ = $1 % P; rpn += std::to_string($1 % P) + " ";}
    | LEFT_P as RIGHT_P { $$ = $2;};

exponent_as: exponent_as PLUS exponent_md { $$ = add($1, $3, Q); exponent_rpn += "+ ";}
           | exponent_as MINUS exponent_md { $$ = sub($1, $3, Q); exponent_rpn += "- ";}
           | exponent_md { $$ = $1; }

exponent_md: exponent_md MUL exponent_n { $$ = mul($1, $3, Q); exponent_rpn += "* ";}
           | exponent_md DIV exponent_n { 
           exponent_rpn += "/ ";
           auto r = div($1, $3, Q);
               if (r.has_value()) {
                   $$ = r.value();
               } else {
                    semantic_error = true;
               }
           }
           | exponent_n { $$ = $1; }

exponent_n: MINUS exponent_n { $$ = neg($2, Q); exponent_rpn += "~ ";}
          | exponent_term { $$ = $1;}

exponent_term: VAL { $$ = $1 % Q; exponent_rpn += std::to_string($1 % Q) + " ";}
             | LEFT_P exponent_as RIGHT_P { $$ = $2;};
%%

void yyerror(const char* s) {
    std::cout << "Error: " << s << std::endl;
}

int main() {
    return yyparse();
}

