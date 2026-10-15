fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u64);
    if n == 2 || n == 3 {
        wln!(out, "NO SOLUTION");
        return
    }
    for i in (2..=n).step_by(2) {
        wln!(out, "{i}");
    }
    for i in (1..=n).step_by(2) {
        wln!(out, "{i}");
    }
}
