use std::cell::Cell;

struct Guard<'a>(&'a Cell<u32>);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

fn work(early: bool, drops: &Cell<u32>) -> u32 {
    let _guard = Guard(drops);
    if early {
        return 10;
    }
    20
}

fn main() {
    let drops = Cell::new(0);
    assert_eq!(work(true, &drops), 10);
    assert_eq!(work(false, &drops), 20);
    assert_eq!(drops.get(), 2);
}
