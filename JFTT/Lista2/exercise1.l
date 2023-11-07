%{
    unsigned int words = 0;
    unsigned int lines = 0;
%}

%%
^[ \t]*\n ;
[^ \t\n]([ \t]+)[^ \t\n] { printf("%c %c", yytext[0], yytext[strlen(yytext) - 1]); words++;}
[^ \t\n] { ECHO; }
\n {ECHO; words++; lines++;}
. ;
%%

int yywrap(){}

int main() {
    yylex();

    fprintf(stderr, "\nwords:  %d\nlines: %d\n", words, lines);
     
    return 0;
}

