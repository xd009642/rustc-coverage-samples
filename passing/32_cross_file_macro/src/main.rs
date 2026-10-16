#[macro_use]
mod macros;

fn choose(flag: bool) -> u32 {
    branch!(flag, 11 + 1, 22 + 2)
}

fn main() {
    assert_eq!(choose(false), 24);
}
