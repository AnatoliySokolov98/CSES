#include <cstdint>
#include <iostream>
#include <string>
#include <vector>

#include "template.hpp"

int main() {
    std::string input;
    std::cin >> input;
    std::vector<std::int64_t> counts(26, 0);
    for (auto c: input) {
        ++counts[c - 'A'];
    }
    int odds = 0;
    int odd_index = 26;
    for (auto c : counts) {
        if (c % 2 == 1) {
            odds++;
        }
    }
    if (odds > 1) {
        std::cout << "NO SOLUTION\n";
        return 0;
    }
    size_t r = input.size() - 1;
    size_t l = 0;
    std::string res(input.size(), 'A');
    for (size_t i = 0; i < 26; ++i) {
        auto count = counts[i];
        if (count % 2 == 1) {
            odd_index= i;
        }
        auto half = count / 2;
        for (std::int64_t j = 0; j < half; ++j) {
            res[l++] = 'A' + i;
            res[r--] = 'A' + i;
        }
    }
    if (odd_index != 26) {
        res[l] = odd_index + 'A';
    }
    std::cout << res;
}
