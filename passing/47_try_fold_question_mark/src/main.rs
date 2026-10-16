use std::{cell::Cell, num::ParseIntError};

fn sum(input: &[&str], visited: &Cell<u32>, accepted: &Cell<u32>) -> Result<u32, ParseIntError> {
    input.iter().try_fold(0, |total, text| {
        visited.set(visited.get() + 1);
        let number = text.parse::<u32>()?;
        accepted.set(accepted.get() + 1);
        Ok(total + number)
    })
}

fn main() {
    let visited = Cell::new(0);
    let accepted = Cell::new(0);
    assert!(sum(&["2", "bad", "9"], &visited, &accepted).is_err());
    assert_eq!((visited.get(), accepted.get()), (2, 1));
}
