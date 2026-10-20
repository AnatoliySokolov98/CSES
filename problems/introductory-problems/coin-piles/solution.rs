fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let t = read!(it, usize);
    for _ in 0..t {
        solve_once(it, out);
    }
}

fn solve_once(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let (a, b) = read!(it, usize, usize);
    let fact = ((a + b) % 3 == 0) && (a * 2 >= b) && (b * 2 >= a);
    if fact {
        wln!(out, "YES");
    } else {
        wln!(out, "NO");
    }
}
