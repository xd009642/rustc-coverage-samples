# rustc coverage samples

The aim of this repo is to find places where coverage counters need improvement in rustc. This is places where:

1. Coverage counters are not created that should be
2. Coverage counters are created where they shouldn't be 

The initial coverage counter implementation did it on the rustc MIR stage. Going from source->HIR->THIR->MIR means
that information is lost causing issues in coverage. Recent work has moved some of the coverage to the THIR stage
this is improving but it feels useful to start to build up a selection of test cases where coverage falls short
to then assess improvements to the coverage implementation.

The way I intend this to work is creating a number of separate MRE rust programs where there are lines present
that should be covered and are not. 

An initial one where we get no coverage counters is:

```rust
#[tracing::instrument]
pub async fn fetch(id: u64) -> Result<String, Error> {
    let row = db_get(id).await?;
    Ok(row.name)
}
```

This one is one found myself through hitting it a lot in production systems. All others have been
found via a codex adversial exploration of language constructs and expected coverage regions.

All code and test harnesses are generated via LLM, written prose is handcrafted.

## Running the samples

The workspace uses Rust 1.98 or newer and edition 2024. Install `cargo-llvm-cov`
and the `llvm-tools-preview` rustup component. cargo-llvm-cov is used because it
generates the LLVM reports with the full region tracking unlike tools like tarpaulin
which condense this to line coverage.

```sh
# All cases, including the uninstrumented counterpart to case 01.
python3 scripts/assess.py --toolchain stable --output target/assessment

# One idea, on a chosen installed compiler.
python3 scripts/assess.py 01_tracing_async --toolchain nightly --output target/assessment-nightly

# Inspect the original LLVM export or a source-annotated report directly.
cargo llvm-cov run --locked -p sample_01_tracing_async --bin sample_01_tracing_async --text
```

I've split samples into a passing and failing folder. Passing ones are trhere to catch 
potential regressions and failing are samples where coverage is incorrect. Each
sample is a standalone binary which should be simple to run (hopefully).

## Falsely Covered Samples

Samples where they report coverage when the code is uncovered feels like the most
concerning case to me. These samples at time of writing are:

| Case | Code incorrectly marked covered | Source lines | 
| --- | --- | --- | --- | --- |
| [13: unwind continuation](failing/13_unwind_continuation/src/main.rs) | Statement after a panicking call | 14 | 
| [23: partial tuple](failing/23_bounds_partial_tuple/src/main.rs) | Later tuple element after a bounds panic | 14 | 
| [24: exit counter underflow](failing/24_exit_counter_underflow/src/main.rs) | Never-taken arms after a call exits the process | 22, 24 |
| [30: panicking destructor](failing/30_panicking_drop/src/main.rs) | Statement after a panicking destructor | 21 | 
| [38: destructuring assignment](failing/38_destructuring_assignment/src/main.rs) | Statement after a later assignment target panics | 11 |
| [39: overloaded addition](failing/39_overloaded_add_unwind/src/main.rs) | Statement and result after overloaded `+` panics | 22, 23 |
| [40: unwinding condition](failing/40_unwinding_condition/src/main.rs) | `else` body when the condition panics before selecting either arm | 19 |
| [45: await unwind](failing/45_await_unwind/src/main.rs) | Continuation after an awaited future panics | 18, 19 |
| [46: division unwind](failing/46_division_unwind/src/main.rs) | Continuation after division by zero | 9, 10 |
| [49: overloaded indexing](failing/49_index_trait_unwind/src/main.rs) | Continuation after overloaded indexing panics | 26, 27 |
| [50: formatting unwind](failing/50_format_display_unwind/src/main.rs) | Statement after `Display::fmt` panics during formatting | 22 |
