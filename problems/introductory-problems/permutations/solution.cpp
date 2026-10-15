#include "template.hpp"

#include <iostream>
#include <cstdint>

int main() {
    std::int64_t n;
    std::cin >> n;
    if (n == 2 || n == 3) {
        std::cout << "NO SOLUTION\n";
        return 0;
    }
    for (std::int64_t i = 2; i <= n; i += 2) {
        std::cout << i << "\n";
    }
    for (std::int64_t i = 1; i <= n; i += 2) {
        std::cout << i << "\n";
    }
}
