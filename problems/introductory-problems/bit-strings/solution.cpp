#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t modulo = 1'000'000'007;
    std::int64_t res = 1;
    std::int64_t n;
    std::cin >> n;
    for (std::int64_t i = 0; i < n; ++i) {
        res = res << 1;
        res %= modulo;
    }
    std::cout << res;
}
