%{
#include <stdbool.h>
    unsigned int words = 0;
    unsigned int lines = 0;
    bool ended_with_new_line = true;
%}

%%
^[ \t]*\n ;
[^ \t\n]([ \t]+)[^ \t\n] { printf("%c %c", yytext[0], yytext[strlen(yytext) - 1]); words++; ended_with_new_line = false;}
[^ \t\n] { ECHO; ended_with_new_line = false;}
\n {ECHO; words++; lines++; ended_with_new_line = true;}
. ;
%%

int yywrap(){}

int main() {
    yylex();
    
    if (!ended_with_new_line) {
        words += 1;
    }

    fprintf(stderr, "\nwords:  %d\nlines: %d\n", words, lines);
     
    return 0;
}

