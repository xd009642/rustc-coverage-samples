use std::cell::Cell;

fn choose(input: &[u32], checks: &Cell<u32>) -> u32 {
    match input {
        [value] | [_, value]
            if {
                checks.set(checks.get() + 1);
                *value > 3 && *value < 10
            } =>
        {
            *value + 1
        }
        _ => 0,
    }
}

fn main() {
    let checks = Cell::new(0);
    assert_eq!(choose(&[0, 7], &checks), 8);
    assert_eq!(checks.get(), 1);
}
