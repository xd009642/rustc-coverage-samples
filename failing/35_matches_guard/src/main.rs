fn accepts(input: Result<u32, u32>, checks: &mut u32) -> bool {
    matches!(input, Ok(value) | Err(value) if {
        *checks += 1;
        value > 4
    })
}

fn main() {
    let mut checks = 0;
    assert!(accepts(Err(7), &mut checks));
    assert_eq!(checks, 1);
}
