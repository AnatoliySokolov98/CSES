// One type returns a value; multiple types return a tuple.
#[macro_export]
macro_rules! read {
    ($it:expr, $ty:ty $(,)?) => {{
        let token = ($it).next().expect("No more input tokens");
        token.parse::<$ty>()
            .unwrap_or_else(|_| panic!("Failed to parse token: {token}"))
    }};
    ($it:expr, $first:ty, $($rest:ty),+ $(,)?) => {{
        let it = ($it).by_ref();
        ($crate::read!(it, $first), $($crate::read!(it, $rest)),+)
    }};
}

#[macro_export]
macro_rules! w {
    ($out:expr, $($args:tt)*) => {{
        #[allow(unused_imports)]
        use std::io::Write as _;
        write!($out, $($args)*).expect("Failed to write output");
    }};
}

#[macro_export]
macro_rules! wln {
    ($out:expr $(, $($args:tt)*)?) => {{
        #[allow(unused_imports)]
        use std::io::Write as _;
        writeln!($out $(, $($args)*)?).expect("Failed to write output");
    }};
}

// Prints a 1D collection with spaces between elements and a final newline.
// Collections are borrowed, so they remain usable afterward.
#[macro_export]
macro_rules! w_vec {
    ($out:expr, $values:expr $(,)?) => {{
        #[allow(unused_imports)]
        use std::io::Write as _;
        let out = ($out).by_ref();
        for (index, value) in ($values).iter().enumerate() {
            if index > 0 {
                $crate::w!(*out, " ");
            }
            $crate::w!(*out, "{value}");
        }
        $crate::wln!(*out);
    }};
}
