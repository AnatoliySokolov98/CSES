fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let t = read!(it, u64);
    for _ in 0..t {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (r, c) = read!(it, i64, i64);
    let diag = r.max(c);
    let square = (diag * diag) - diag + 1;
    if diag % 2 == 0 {
        wln!(out, "{}", square + r - c);
    } else {
        wln!(out, "{}", square - r + c);
    }
}
