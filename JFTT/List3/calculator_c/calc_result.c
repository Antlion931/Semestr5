#include "calc_result.h"

CalcResult new_calc_result(char *rpn, GF value) {
    return (CalcResult){.rpn = rpn, .value = value, }
}

CalcResult ignore(CalcResult self);

CalcResult broken(CalcResult self);

CalcResult from(GF value);

void display_calc_result();

