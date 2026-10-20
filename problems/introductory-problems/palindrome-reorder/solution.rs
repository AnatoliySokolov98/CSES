fn solve(it: &mut std::str::SplitWhitespace<'_>, out: &mut impl std::io::Write) {
    let s = read!(it, String).into_bytes();
    let mut counts = [0; 26];
    for &c in s.iter() {
        counts[(c - b'A') as usize] += 1;
    }
    let mut odd_count = 0;
    let mut odd_index = 26;
    for i in 0..counts.len() {
        if counts[i] % 2 == 1 {
            odd_count += 1;
            odd_index = i;
        }
    }
    if odd_count > 1 {
        wln!(out, "NO SOLUTION");
        return;
    }
    let mut res = vec![b'*'; s.len()];
    let mut l = 0;
    let mut r = s.len() - 1;
    for (i, &v) in counts.iter().enumerate() {
        let half = v / 2;
        for _ in 0..half {
            res[l] = b'A' + (i as u8);
            res[r] = b'A' + (i as u8);
            l += 1;
            r -= 1;
        }
    }
    if odd_index != 26 {
        res[l] = b'A' + (odd_index as u8);
    }
    wln!(out, "{}", String::from_utf8(res).unwrap());
}
