macro_rules! branch {
    ($condition:expr, $yes:expr, $no:expr) => {
        if $condition { $yes } else { $no }
    };
}
