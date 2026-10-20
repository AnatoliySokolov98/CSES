#include "template.hpp"
#include <iostream>
#include <cstdint>

void solve() {
    std::int64_t a;
    std::int64_t b;
    std::cin >> a >> b;
    if (((a + b) % 3 == 0) && (a * 2 >= b) && (b * 2) >= a) {
        std::cout << "YES\n";
    } else {
        std::cout << "NO\n";
    }
}

int main() {
    std::int64_t t;
    std::cin >> t;
    for (std::int64_t i = 0; i < t; ++i) {
        solve();
    }
}

