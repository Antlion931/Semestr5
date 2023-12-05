#include "gf.h"
#include <stdio.h>

const int64_t P = 1234577;

GF new_GF(int64_t x) {
    return (GF){.value = (x % P + P) % P, .error = 0};
}

GF pow_GF(GF base, GF n) {
    GF result = new_GF(1);
    int64_t exponent = n.value;
    while (exponent > 0) {
        if (exponent & 1) {
            result = (GF){.value = (result.value * base.value) % P, .error = 0};
        }
        base = (GF){.value = (base.value * base.value) % P, .error = 0};
        exponent >>= 1;
    }
    return result;
}

GF inv_GF(GF a) {
    if (a.value == 0) {
        return (GF){.value = 0, .error = 1};
    } else {
        return pow_GF(a, new_GF(P - 2));
    }
}

GF add_GF(GF a, GF b) {
    if (a.error || b.error) {
        return (GF){.value = 0, .error = 1};
    } else {
        return new_GF((a.value + b.value) % P);
    }
}

GF sub_GF(GF a, GF b) {
    if (a.error || b.error) {
        return (GF){.value = 0, .error = 1};
    } else {
        return new_GF((a.value - b.value + P) % P);
    }
}

GF mul_GF(GF a, GF b) {
    if (a.error || b.error) {
        return (GF){.value = 0, .error = 1};
    } else {
        return new_GF((a.value * b.value) % P);
    }
}

GF neg_GF(GF a) {
    if (a.error) {
        return (GF){.value = 0, .error = 1};
    } else {
        return new_GF(P - a.value);
    }
}

GF div_GF(GF a, GF b) {
    return mul_GF(a, inv_GF(b));
}

void display_GF(GF *a) {
    if (a->error) {
        printf("Error");
    } else {
        printf("%lld", (long long int)a->value);
    }
}

