#include <algorithm>
#include <cstdint>
#include <iostream>

#include "template.hpp"

int main() {
    std::int64_t res = 0;
    std::int64_t best = 0;
    std::int64_t n;
    std::cin >> n;
    for (std::int64_t i = 0; i < n; ++i) {
        std::int64_t curr;
        std::cin >> curr;
        best = std::max(best, curr);
        res += best - curr;
    }
    std::cout << res << "\n";
}
