fn work() -> u32 {
    let mut total = 0;
    'outer: for i in 0..3 {
        for j in 0..3 {
            if j == i {
                continue 'outer;
            }
            total += 1;
        }
        total += 100;
    }
    total
}

fn main() {
    assert_eq!(work(), 3);
}
