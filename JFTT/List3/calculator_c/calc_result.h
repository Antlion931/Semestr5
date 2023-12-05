#ifndef CALC_H
#define CALC_H

#include "gf.h"
#include <stdbool.h>

typedef struct {
    char *rpn;
    bool rpn_error;
    bool ignore;
    GF value;
} CalcResult;

CalcResult new_calc_result(char *rpn, GF value);
CalcResult ignore(CalcResult self);
CalcResult broken(CalcResult self);
CalcResult from(GF value);
void display_calc_result();

#endif

