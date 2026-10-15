fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let mut n = read!(it, i64);
    loop {
        wln!(out, "{n}");
        if n == 1 {
            return;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }
}
