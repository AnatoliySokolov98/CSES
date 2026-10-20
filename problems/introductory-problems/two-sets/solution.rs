fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let n = read!(it, u64);
    let total = n * (n + 1) / 2;
    if total % 2 == 1 {
        wln!(out, "NO");
        return;
    }
    let mut half = total / 2;
    let mut first_nums = Vec::new();
    let mut second_nums = Vec::new();
    for i in (1..=n).rev() {
        if i <= half {
            first_nums.push(i);
            half -= i;
        } else {
            second_nums.push(i);
        }
    }
    wln!(out, "YES");
    wln!(out, "{}", first_nums.len());
    w_vec!(out, first_nums);
    wln!(out, "{}", second_nums.len());
    w_vec!(out, second_nums);
}
