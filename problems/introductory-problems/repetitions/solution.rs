fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let input = read!(it, String).into_bytes();
    let mut curr = 1;
    let mut res = 1;
    for i in 1..input.len() {
        if input[i] != input[i - 1] {
            curr = 0;
        }
        curr += 1;
        res = res.max(curr);
    }
    wln!(out, "{res}");
}
