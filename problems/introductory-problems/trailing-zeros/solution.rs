fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u64);
    let mut res = 0;
    let mut five = 5;
    while five <= n {
        res += n / five;
        five *= 5;
    }
    wln!(out, "{}", res);
}
