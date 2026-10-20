fn choose(input: Result<u32, u32>, checks: &mut u32) -> u32 {
    match input {
        Ok(value) | Err(value)
            if {
                *checks += 1;
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
    let mut checks = 0;
    assert_eq!(choose(Err(7), &mut checks), 17);
    assert_eq!(checks, 1);
}
