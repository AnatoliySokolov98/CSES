#include <cstdint>
#include <vector>

#include "template.hpp"

int main() {
    std::int64_t n;
    std::cin >> n;
    std::int64_t total = n * (n + 1) / 2;
    if (total % 2 == 1) {
        std::cout << "NO";
        return 0;
    }
    std::int64_t half = total / 2;
    std::vector<std::int64_t> first;
    std::vector<std::int64_t> second;
    for (std::int64_t i = n; i > 0; --i) {
        if (i <= half) {
            first.push_back(i);
            half -= i;
        } else {
            second.push_back(i);
        }
    }
    std::cout << "YES\n";
    std::cout << first.size() << "\n";
    for (auto num : first) {
        std::cout << " " << num;
    }
    std::cout << "\n";
    std::cout << second.size() << "\n";
    for (auto num : second) {
        std::cout << " " << num;
    }
}
