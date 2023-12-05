%{
#define YYSTYPE int
#include<stdio.h>
int yylex();
int yyerror(char*);
%}
%token PLUS
%token MINUS
%token MUL
%token DIV
%token POW
%token LEFT_P
%token RIGHT_P
%token VAL
%token COM
%%
expr: first_tier | COM
%%
int yyerror(char *s)
{
    printf("%s\n",s);	
    return 0;
}

int main()
{
    yyparse();
    return 0;
}
