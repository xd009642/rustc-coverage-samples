use std::cell::Cell;

fn choose(input: Result<u32, u32>, checks: &Cell<u32>) -> u32 {
    match input {
        Ok(value) | Err(value)
            if {
                checks.set(checks.get() + 1);
                value > 4
            } =>
        {
            value + 10
        }
        Ok(value) => value + 20,
        Err(value) => value + 30,
    }
}

fn main() {
    let checks = Cell::new(0);
    assert_eq!(choose(Err(7), &checks), 17);
    assert_eq!(checks.get(), 1);
}
