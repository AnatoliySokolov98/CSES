#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t res = 0;
    std::int64_t five = 5;
    std::int64_t n;
    std::cin >> n;
    while (five <= n) {
        res += n / five;
        five *= 5;
    }
    std::cout << res << "\n";
}
