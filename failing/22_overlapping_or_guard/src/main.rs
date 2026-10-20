fn choose(value: u32, checks: &mut u32) -> u32 {
    match value {
        1 | _
            if {
                *checks += 1;
                false
            } =>
        {
            10
        }
        _ => 20,
    }
}

fn main() {
    let mut checks = 0;
    assert_eq!(choose(1, &mut checks), 20);
    assert_eq!(checks, 2);
}
