fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut res = 1;
    let n = read!(it, u64);
    let modulo = 1_000_000_007;
    for _ in 0..n {
        res = (res * 2) % modulo;
    }
    wln!(out, "{}", res);
}
