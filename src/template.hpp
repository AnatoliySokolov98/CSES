#ifndef CSES_TEMPLATE_HPP
#define CSES_TEMPLATE_HPP

#include <cstddef>
#include <cstdint>
#include <iostream>
#include <limits>

// Runs before main: fast cin/cout. Flush manually in interactive problems.
[[maybe_unused]] static const bool fast_io = [] {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    return true;
}();

// Safe infinities: INF + INF still fits in the type.
inline constexpr int INF = 1'000'000'000;
inline constexpr int NEG_INF = -1'000'000'000;
inline constexpr std::int64_t INF64 = 1'000'000'000'000'000'000;
inline constexpr std::int64_t NEG_INF64 = -1'000'000'000'000'000'000;

#endif  // CSES_TEMPLATE_HPP
