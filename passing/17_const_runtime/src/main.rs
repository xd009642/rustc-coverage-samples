const fn choose(flag: bool) -> u32 {
    if flag { 11 } else { 22 }
}

const VALUE: u32 = choose(false);

fn main() {
    assert_eq!(VALUE, 22);
    assert_eq!(choose(std::hint::black_box(true)), 11);
}
