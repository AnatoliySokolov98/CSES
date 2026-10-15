#include <algorithm>
#include <cstdint>
#include <iostream>
#include <string>

#include "template.hpp"

int main() {
    std::string input;
    std::cin >> input;
    std::int64_t curr = 1;
    std::int64_t res = 1;
    for (size_t i = 1; i < input.size(); ++i) {
        if (input[i] != input[i - 1]) {
            curr = 0;
        };
        ++curr;
        res = std::max(res, curr);
    }
    std::cout << res << "\n";
}
