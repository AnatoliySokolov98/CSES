#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t n;
    std::cin >> n;
    for (std::int64_t i = 1; i <= n; ++i) {
        std::cout << (i * i) * (i * i - 1) / 2 - (i - 1) * (i - 2) * 4 << "\n";
    }
}
