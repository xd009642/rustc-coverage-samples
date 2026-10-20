use std::panic::{AssertUnwindSafe, catch_unwind};

struct Bomb<'a>(&'a mut u32);

impl Drop for Bomb<'_> {
    fn drop(&mut self) {
        *self.0 += 1;
        if std::env::args().len() == 1 {
            panic!("intentional destructor panic");
        }
    }
}

fn work(drops: &mut u32, after: &mut u32) {
    {
        let _bomb = Bomb(drops);
    }
    *after += 1;
}

fn main() {
    let mut drops = 0;
    let mut after = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| work(&mut drops, &mut after))).is_err());
    assert_eq!(drops, 1);
    assert_eq!(after, 0);
}
