fn sum_positive(values: &[i32]) -> i32 {
    values
        .iter()
        .filter_map(|&value| if value > 0 { Some(value + 1) } else { None })
        .sum()
}

fn main() {
    assert_eq!(sum_positive(&[-1, 2, 3]), 7);
}
