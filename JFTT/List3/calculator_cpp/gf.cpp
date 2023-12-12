#include "gf.hpp"
#include <cstdint>


std::tuple<int64_t, int64_t, int64_t> extended_gcd(int64_t a, int64_t b) {
    if (a == 0 && b == 0) {
        throw "Tryied to calculate gcd with zeros";
    }

    std::tuple<int64_t, int64_t> last_two_r = std::make_tuple(a, b);
    std::tuple<int64_t, int64_t> last_two_s = std::make_tuple(1, 0);
    std::tuple<int64_t, int64_t> last_two_t = std::make_tuple(0, 1);

    while (std::get<1>(last_two_r) != 0) {
        int64_t q = std::get<0>(last_two_r) / std::get<1>(last_two_r);

        last_two_r = std::make_tuple(std::get<1>(last_two_r), std::get<0>(last_two_r) - q * std::get<1>(last_two_r));

        last_two_s = std::make_tuple(std::get<1>(last_two_s), std::get<0>(last_two_s) - q * std::get<1>(last_two_s));

        last_two_t = std::make_tuple(std::get<1>(last_two_t), std::get<0>(last_two_t) - q * std::get<1>(last_two_t));

    }

    return std::make_tuple(std::get<0>(last_two_r), std::get<0>(last_two_s), std::get<0>(last_two_t));
}

int64_t pow(int64_t a, int64_t b, int64_t p) {
    int64_t n = b;
    int64_t result = 1;

    while (n > 0) {
        result = (result * a) % p;
        n--;
    }

    return result;
}

std::optional<int64_t> inv(int64_t a, int64_t p) {
    if (a == 0) {
        return std::nullopt;
    }

    std::tuple<int64_t, int64_t, int64_t> gcd = extended_gcd(a, p);

    if (std::get<0>(gcd) != 1) {
        return std::nullopt;
    }

    return std::optional<int64_t>((std::get<1>(gcd) % p + p) % p);
}

int64_t add(int64_t a, int64_t b, int64_t p) {
    return (a + b) % p;
}

int64_t sub(int64_t a, int64_t b, int64_t p) {
    return (a + p - b) % p;
}

int64_t mul(int64_t a, int64_t b, int64_t p) {
    return (a * b) % p;
}

int64_t neg(int64_t a, int64_t p) {
    return (p - a) % p;
}

std::optional<int64_t> div(int64_t a, int64_t b, int64_t p) {
    auto i = inv(b, p);

    if (!i.has_value()) {
        return std::nullopt;
    } 

    return std::optional<int64_t>(mul(a, i.value(), p));
}

