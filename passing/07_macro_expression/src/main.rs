macro_rules! choose {
    ($condition:expr, $yes:expr, $no:expr) => {
        if $condition { $yes } else { $no }
    };
}

fn evaluate(flag: bool) -> u32 {
    choose!(flag, 10 + 1, 20 + 2)
}

fn main() {
    assert_eq!(evaluate(true), 11);
}
