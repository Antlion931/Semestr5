%option noyywrap
%option yylineno
%{
#include "calc_y.h"
int yylex();
%}
%%
[ \t]+ 	;
\+ 	{ return PLUS; }
\- 	{ return MINUS; }
\*	{ return MUL; }
\/ 	{ return DIV; }
\^  { return POW: }
\(	{ return LNAW; }
\)	{ return PNAW; }
[0-9]+	{ yyval = atoi(yytext); return VAL; }
#.{0,} {return COM; }
.	{ return ERROR; }
%%
