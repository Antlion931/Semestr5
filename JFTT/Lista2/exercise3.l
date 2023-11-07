%{
#include <stdbool.h> 
bool documentation = false;

void doc_print(char* text) {
    if (documentation) {
        printf("%s", text);
    }
}
%}

LB  (\\\n)

%s FREE
%s MULTILINE_COMMENT
%s SINGLELINE_COMMENT
%s STRING

%s MULTILINE_DOC_COMMENT
%s SINGLELINE_DOC_COMMENT

%%
  BEGIN(FREE);
<FREE>\/{LB}*\*{LB}*[\*\!] {BEGIN(MULTILINE_DOC_COMMENT); doc_print(yytext);}
<MULTILINE_DOC_COMMENT>\*{LB}*\/ {BEGIN(FREE); doc_print(yytext);}
<MULTILINE_DOC_COMMENT>.|\n {doc_print(yytext);}

<FREE>\/{LB}*\/{LB}*[\/\!] {BEGIN(SINGLELINE_DOC_COMMENT); doc_print(yytext);}
<SINGLELINE_DOC_COMMENT>{LB} {doc_print(yytext);}
<SINGLELINE_DOC_COMMENT>\n {BEGIN(FREE); ECHO;}
<SINGLELINE_DOC_COMMENT>. {doc_print(yytext);}

<FREE>\/{LB}*\* BEGIN(MULTILINE_COMMENT);
<MULTILINE_COMMENT>\*{LB}*\/ BEGIN(FREE);
<MULTILINE_COMMENT>.|\n ;

<FREE>\/{LB}*\/ BEGIN(SINGLELINE_COMMENT);
<SINGLELINE_COMMENT>{LB} ;
<SINGLELINE_COMMENT>\n {BEGIN(FREE); ECHO;}
<SINGLELINE_COMMENT>. ;

<FREE>\" {BEGIN(STRING); ECHO;}
<STRING>\\\" ECHO;
<STRING>\" {BEGIN(FREE); ECHO;}

%%

int yywrap(){}


int main(int argc, char** argv) {
    if (argc > 2) {
        fprintf(stderr, "There should be at most one argument given (-d to not delete documentation comments)\n" );
    }

    if (argc > 1) {
        if (strcmp(argv[1], "-d") != 0) {
            fprintf(stderr, "There should be at most one argument given (-d to not delete documentation comments)\n" );
        } else {
            documentation = true;
        }
    }

    yylex();
     
    return 0;
}


