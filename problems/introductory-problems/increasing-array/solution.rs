fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, i64);
    let mut res = 0;
    let mut best = 0;
    for _ in 0..n {
        let curr = read!(it, i64);
        best = curr.max(best);
        res += best - curr;
    }
    wln!(out, "{res}");
}
