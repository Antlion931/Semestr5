%{
#include <math.h>
#include <limits.h>
#include <stdbool.h>
#define START_SIZE 7 // should be equal to 2^n - 1
bool in_error = false;
int* stack;
unsigned int actual_size = 0;
unsigned int size = START_SIZE;

void push(int element);
int pop();

void addition();
void subtracion();
void multipication();
void division();
void modulo();
void power();

void print_stack();
void end_step();
void unknow_symbol(char symbol);
%}

%%
\-?[0-9]+ {push(atoi(yytext));}
\+ {addition();}
\- {subtracion();}
\* {multipication();}
\/ {division();}
\% {modulo();}
\^ {power();}
\n {end_step();}
[ \t] ;
. {unknow_symbol(yytext[0]);} 
%%

void push(int element) {
    if (actual_size == size) {
        if (size == UINT_MAX) {
            fprintf(stderr, "Memory limit reached\n");
            exit(1);
        }
        size = (size << 1) + 1;
        stack = realloc(stack, size * sizeof(int));
        if (!stack) {
            fprintf(stderr, "Memory limit reached\n");
            exit(1);
        }
    }

    stack[actual_size] = element;
    actual_size++;
}

int pop() {
    if (actual_size == 0) {
        fprintf(stderr, "There's nothing to pop!\n");
        exit(1);
    }

    return stack[--actual_size];
}

void addition() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make addition\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();

    if (a >= 0) {
        if (b > INT_MAX - a) {
            fprintf(stderr, "Overflow during addition\n");
            in_error = true;
            return;
        }
        push(a + b);
    } else {
        if (b < INT_MIN - a) {
            fprintf(stderr, "Underflow during addition\n");
            in_error = true;
            return;
        } 
        push(a + b);
    }
}

void subtracion() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make subtracion\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();

    if (b >= 0) {
        if (a < INT_MIN + b) {
            fprintf(stderr, "Underflow during subtracion\n");
            in_error = true;
            return;
        }
        push(a - b);
    } else {
        if (a > INT_MAX + b) {
            fprintf(stderr, "Overflow during subtracion\n");
            in_error = true;
            return;
        } 
        push(a - b);
    }
}

void multipication() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make multipicationn\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();
    
    // HERE should be test to overflow in multipication
    
    push(a * b);
}

void division() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make division\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();
    
    if (b == 0) {
        fprintf(stderr, "Can't divide by zero\n");
        in_error = true;
        return;
    } 
    push(a / b);
}

void modulo() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make modulo\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();
    
    if (b == 0) {
        fprintf(stderr, "Can't divide by zero\n");
        in_error = true;
        return;
    } 
    push(a % b);
}

void power() {
    if (in_error) {
        return;
    }

    if (actual_size < 2) {
        fprintf(stderr, "Too few numbers on stack to make multipicationn\n");
        in_error = true;
        return;
    }

    int b = pop();
    int a = pop();

    //Here shouldbe check for overflow in power

    push(pow(a, b));
}

void print_stack() {
    for (int i = 0; i < actual_size; i++) {
        printf("%d ", stack[i]);
    }

    printf("\n");
}

void end_step() {
    if (!in_error) {
        if (actual_size == 1) {
            printf("= %i\n", pop());
        } else if (actual_size > 1 ) {
            fprintf(stderr, "Too few operators\n");
        }
    }
    
    in_error = false;
    actual_size = 0;
}

void unknow_symbol(char symbol) {
    if (!in_error) {
        in_error = true;
        fprintf(stderr, "Unknow symbol: %c\n", symbol);
    }
}

int yywrap(){}

int main() {
    stack = malloc(size * sizeof(int));
    if (!stack) {
        fprintf(stderr, "Could not malloc with start size = %d\n", START_SIZE);
        exit(1);
    }
    yylex();
    return 0;
}


