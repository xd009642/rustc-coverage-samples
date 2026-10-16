use std::cell::Cell;

trait Value {
    fn choose(&self, flag: bool, calls: &Cell<u32>) -> u32 {
        calls.set(calls.get() + 1);
        if flag { 11 } else { 22 }
    }
}

struct Worker;
impl Value for Worker {}

fn dispatch(value: &dyn Value, calls: &Cell<u32>) -> u32 {
    value.choose(false, calls)
}

fn main() {
    let calls = Cell::new(0);
    let value: &dyn Value = std::hint::black_box(&Worker);
    assert_eq!(dispatch(value, &calls), 22);
    assert_eq!(calls.get(), 1);
}
