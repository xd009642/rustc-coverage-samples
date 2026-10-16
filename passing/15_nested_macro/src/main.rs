macro_rules! inside {
    ($body:expr) => {{ $body }};
}
macro_rules! outside {
    ($condition:expr, $yes:expr, $no:expr) => {
        inside!(if $condition { $yes } else { $no })
    };
}

fn select(flag: bool) -> u32 {
    outside!(flag, 11 + 1, 22 + 2)
}

fn main() {
    assert_eq!(select(true), 12);
}
