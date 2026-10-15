# CSES in C++ and Rust

All 400 problems across 18 categories from [CSES](https://cses.fi/problemset/), with public sample inputs and outputs. Every problem can be solved in C++, Rust, or both, side by side. Requires `clang++` with C++23 support, Rust (`rustc`, Cargo), and Python 3; no Python packages are needed.

```
problems/introductory-problems/weird-algorithm/
  README.md        task link
  input.txt        scratch input (starts as the first sample)
  solution.cpp
  solution.rs
  tests/           sample-N.in / sample-N.out, plus any pairs you add
```

## The `cses` helper

```sh
./cses list                                  # every problem, with a mark per solved language
./cses list --todo rust                      # only problems not yet solved in Rust
./cses test weird-algorithm                  # samples, in every language you've solved
./cses test 1068 -l cpp                      # just one language (cpp/c++ or rust/rs)
./cses test 1068 --debug                     # C++ sanitizers / Rust overflow checks
./cses run weird-algorithm                   # reads the problem's input.txt
./cses run 1068 --input my.txt               # custom path relative to your shell
./cses run 1068 --input -                    # read stdin (given once, fed to each language)
./cses submit weird-algorithm                # write the single file(s) to paste into CSES
./cses sync                                  # after editing problems/index.json
```

A problem can be named by its slug, CSES ID, folder, or solution file. Naming a solution file picks that language, so `./cses test problems/.../solution.rs` only tests Rust. Otherwise every language whose solution differs from the generated stub is used, and `--lang`/`-l` narrows it to one. Test output shows the time for each case, so you can compare the two languages.

CSES takes one file, so every command first writes `build/submissions/<name>.cpp` or `.rs` and compiles exactly that file, rebuilding only when it changes. Paste the file `submit` prints into CSES, not your solution file, and don't edit generated files. The test command compares whitespace-separated tokens and returns a nonzero status on failure. It is a sample checker, not the CSES judge: constructive problems can have valid outputs that differ from the saved ones. Runs time out after five seconds; override with `--timeout 10`.

## C++

```cpp
#include "template.hpp"

int main() {
    std::int64_t n;
    std::cin >> n;
    std::cout << n * 2 << '\n';
}
```

`src/template.hpp` includes `<cstdint>`, `<iostream>`, and `<limits>`, defines `INF`/`INF64` constants, and turns on fast I/O before `main` runs. `submit` replaces the `#include "template.hpp"` line with the template's contents, so each solution needs exactly one such line. Flags are `-std=c++23 -O2 -Wall -Wextra -Wshadow`. Set `CXX=g++-14` to use another compiler. `--debug` adds `-g`, AddressSanitizer, UndefinedBehaviorSanitizer, and `_GLIBCXX_ASSERTIONS`, which catch out-of-bounds access, signed overflow, and similar bugs.

## Rust

```rust
fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (a, b) = read!(it, usize, usize);
    wln!(out, "{}", a + b);
}
```

Keep helper functions, types, and imports in the solution file. The generated file contains `src/macros.rs`, `src/runner.rs` (which reads all of stdin, calls `solve`, and flushes buffered stdout), and your solution. Read one type for a single value, or several for a tuple. Use `w!`/`wln!` for formatted output and `w_vec!(out, values)` for a space-separated line. Submissions compile with `rustc --edition=2021 -O` to match the judge, so use only the standard library. `--debug` turns on overflow checks and debug assertions. Set `RUSTC` to use another compiler.

rust-analyzer discovers every solution through the small Cargo entry points in `src/bin/`, which `./cses sync` generates. Edit the solution files, not the wrappers.

```sh
cargo check --all-targets
cargo run --bin missing-number < problems/introductory-problems/missing-number/input.txt
cargo test --lib                # checks the shared macros
```

## Interactive problems

Six tasks in Interactive Problems have `"interactive": true` in the index. Their examples are saved as `tests/example-transcript.txt`, not input/output pairs, so `./cses test` rejects them. Run them with `--input -` and one `--lang` against a live interactor, or generate the file with `submit`. In C++, flush after every query. In Rust they use `src/interactive_runner.rs` and `solve(input: &mut impl std::io::BufRead, out: &mut impl std::io::Write)`. Read replies with `input.read_line(...)` and call `out.flush()` after each query.

## VS Code

Open this folder as the workspace. **Ctrl+Shift+B** compiles and runs the open solution file on its `input.txt`. **Terminal → Run Task…** also offers the sample tests for the open file, the tests for both languages, and the debug build. Compiler errors from both languages are clickable. Both formatters run on save: C++ uses `.clang-format`, Rust uses `rustfmt.toml`.

## Adding a problem

Create its folder with `README.md` and test pairs, add its ID, name, category, path, and order to `problems/index.json`, and run `./cses sync`. That creates both solution stubs and the Cargo entry point.
