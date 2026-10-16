use std::{
    cell::Cell,
    panic::{AssertUnwindSafe, catch_unwind},
};

struct Bomb<'a>(&'a Cell<u32>);

impl Drop for Bomb<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
        if std::hint::black_box(true) {
            panic!("intentional destructor panic");
        }
    }
}

fn work(drops: &Cell<u32>, after: &Cell<u32>) {
    {
        let _bomb = Bomb(drops);
    }
    after.set(after.get() + 1);
}

fn main() {
    let drops = Cell::new(0);
    let after = Cell::new(0);
    assert!(catch_unwind(AssertUnwindSafe(|| work(&drops, &after))).is_err());
    assert_eq!(drops.get(), 1);
    assert_eq!(after.get(), 0);
}
