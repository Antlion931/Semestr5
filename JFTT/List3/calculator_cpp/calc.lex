%option noyywrap
%option nounput
%option noinput

%{
#include "calc_y.hpp"

int yylex();
%}

BL \\\n

%%
[ \t]+ 	;
{BL}  ;
^#(.*{BL})*.*$  ;
\+ 	{ return PLUS; }
\- 	{ return MINUS; }
\*	{ return MUL; }
\/ 	{ return DIV; } 
\^  { return POW; }
\(	{ return LEFT_P; }
\)	{ return RIGHT_P; }
[0-9]+	{ yylval= atoi(yytext); return VAL; }
\n     { return EOL; }
.	{ return ERROR; }
%%

