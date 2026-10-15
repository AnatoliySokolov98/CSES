#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t n;
    std::cin >> n;
    while (true) {
        std::cout << n << "\n";
        if (n == 1) {
            break;
        }
        if (n % 2 == 0) {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }
}
