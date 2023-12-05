#ifndef GF_H
#define GF_H

#include <stdint.h>

typedef struct {
    int64_t value;
    int error;
} GF;

GF new_GF(int64_t x);
GF pow_GF(GF base, GF n);
GF inv_GF(GF a);
GF add_GF(GF a, GF b);
GF sub_GF(GF a, GF b);
GF mul_GF(GF a, GF b);
GF neg_GF(GF a);
GF div_GF(GF a, GF b);
void display_GF(GF *a);

#endif

