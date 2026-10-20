fn step(iteration: u32, calls: &mut u32) -> Result<(), u32> {
    *calls += 1;
    if iteration == 3 {
        assert_eq!(*calls, 3);
        std::process::exit(0);
    }
    if iteration > 10 { Err(1) } else { Ok(()) }
}

fn run(calls: &mut u32) -> Result<(), u32> {
    let mut iteration = 0;
    loop {
        iteration += 1;
        let code = match step(iteration, calls) {
            Ok(()) => continue,
            Err(code) => code,
        };
        match code {
            1 | 2 => {
                println!("{code}");
            }
            _ => return Err(code),
        }
    }
}

fn main() {
    let mut calls = 0;
    let result = run(&mut calls);
    panic!("run unexpectedly returned: {result:?}");
}
