fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, i64);
    let nums: Vec<i64> = (1..n).map(|_|read!(it, i64)).collect();
    wln!(out, "{}", n * (n + 1) / 2 - nums.iter().sum::<i64>());
}
