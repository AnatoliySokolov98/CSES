#include <algorithm>
#include <cstdint>
#include <iostream>

#include "template.hpp"

void solve() {
    std::int64_t row;
    std::int64_t col;
    std::cin >> row >> col;
    std::int64_t diag = std::max(row, col);
    std::int64_t square = (diag * diag) - diag + 1;
    if (diag % 2 == 0) {
        std::cout << square + row - col << "\n";
    } else {
        std::cout << square - row + col << "\n";
    }
}

int main() {
    std::int64_t t;
    std::cin >> t;
    for (std::int64_t i = 0; i < t; i++) {
        solve();
    }
}
