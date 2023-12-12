#ifndef GALOIS_FIELD_HPP
#define GALOIS_FIELD_HPP
#include <cstdint>
#include <stdint.h>
#include <tuple>
#include <optional>

std::tuple<int64_t, int64_t, int64_t> extended_gcd(int64_t a, int64_t b);

int64_t pow(int64_t a, int64_t b, int64_t p);

std::optional<int64_t> inv(int64_t a, int64_t p);

int64_t add(int64_t a, int64_t b, int64_t p);

int64_t sub(int64_t a, int64_t b, int64_t p);

int64_t mul(int64_t a, int64_t b, int64_t p);

int64_t neg(int64_t a, int64_t p);

std::optional<int64_t> div(int64_t a, int64_t b, int64_t p);

#endif
