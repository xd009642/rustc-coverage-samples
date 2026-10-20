before simplification

```rust
use std::cell::Cell;

fn step(iteration: u32, calls: &Cell<u32>) -> Result<(), u32> {
    calls.set(calls.get() + 1);
    if iteration == 3 {
        assert_eq!(calls.get(), 3);
        std::process::exit(0);
    }
    if iteration > 10 { Err(1) } else { Ok(()) }
}

fn run(calls: &Cell<u32>) -> Result<(), u32> {
    let mut iteration = 0;
    loop {
        iteration += 1;
        let code = match step(iteration, calls) {
            Ok(()) => continue,
            Err(code) => code,
        };
        match code {
            1 | 2 => {
                std::hint::black_box(code);
            }
            _ => return Err(code),
        }
    }
}

fn main() {
    let calls = Cell::new(0);
    let result = run(&calls);
    panic!("run unexpectedly returned: {result:?}");
}
```
