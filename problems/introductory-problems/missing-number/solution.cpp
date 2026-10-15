#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t n;
    std::cin >> n;
    std::int64_t total = n * (n + 1) / 2;
    for (std::int64_t i = 1; i < n; ++i) {
        std::int64_t curr;
        std::cin >> curr;
        total -= curr;
    }
    std::cout << total << "\n";
}
