fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u64);
    for i in 1..=n {
        wln!(out, "{}", (i * i) * (i * i - 1) / 2 - (i - 1) * (i - 2) * 4);
    }
}
