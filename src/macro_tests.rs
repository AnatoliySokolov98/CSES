use crate::{read, w, w_vec, wln};

#[test]
fn writes_vectors_through_a_borrowed_writer() {
    fn print(out: &mut impl std::io::Write) {
        w_vec!(out, vec![1, 2]);
        for row in &[vec![3, 4], vec![5]] {
            w_vec!(out, row);
        }
        wln!(out, "done");
    }
    let mut buffer = Vec::new();
    print(&mut buffer);
    assert_eq!(buffer, b"1 2\n3 4\n5\ndone\n");
}

#[test]
fn reads_mixed_types_in_order() {
    let mut it = "3 4 hello\n-5".split_whitespace();
    assert_eq!(read!(it, usize, usize, String), (3, 4, "hello".into()));
    assert_eq!(read!(it, i64), -5);
    assert_eq!(it.next(), None);
}

#[test]
fn formats_values_vectors_and_rows() {
    let mut out = Vec::new();
    w!(out, "{} ", 42);
    wln!(out, "hello");
    wln!(out);
    let values = vec![1, 2, 3];
    w_vec!(out, values);
    assert_eq!(values.len(), 3);
    w_vec!(out, Vec::<i32>::new());
    let rows = vec![vec![4, 5], vec![], vec![6]];
    for row in &rows {
        w_vec!(out, row);
    }
    assert_eq!(rows.len(), 3);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "42 hello\n\n1 2 3\n\n4 5\n\n6\n"
    );
}
