use std::{
    ops::Add,
    panic::{AssertUnwindSafe, catch_unwind},
};

struct Number<'a>(&'a mut u32);

impl Add for Number<'_> {
    type Output = u32;
    fn add(self, _: Self) -> u32 {
        *self.0 += 1;
        if std::env::args().len() == 1 {
            panic!("operator panic");
        }
        7
    }
}

fn sum(calls: &mut u32, after: &mut u32) -> u32 {
    let total = Number(calls) + Number(&mut 0);
    *after += 1;
    total + 1
}

fn main() {
    let mut calls = 0;
    let mut after = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| sum(&mut calls, &mut after))).is_err());
    assert_eq!((calls, after), (1, 0));
}
