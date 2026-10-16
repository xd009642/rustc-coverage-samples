# Unexecuted code reported as covered

11 samples report positive coverage counts for expressions that never executed.
The saved stable and installed nightly results agree for every sample below.
Each listed expression should count 0 but reports 1, except the enormous count
in case 24.

Links in the evidence columns open LLVM's annotated source reports, showing the
affected lines and their reported counts. Matching JSON files in the same
directories retain the expected counts, source coordinates, and LLVM regions.

| Case | Code incorrectly marked covered | Source lines | Stable evidence | Nightly evidence |
| --- | --- | --- | --- | --- |
| [13: unwind continuation](failing/13_unwind_continuation/src/main.rs) | Statement after a panicking call | 14 | [Report](evidence/batch2-stable/13_unwind_continuation.txt) | [Report](evidence/batch2-nightly/13_unwind_continuation.txt) |
| [23: partial tuple](failing/23_bounds_partial_tuple/src/main.rs) | Later tuple element after a bounds panic | 14 | [Report](evidence/batch3-stable/23_bounds_partial_tuple.txt) | [Report](evidence/batch3-nightly/23_bounds_partial_tuple.txt) |
| [24: exit counter underflow](failing/24_exit_counter_underflow/src/main.rs) | Never-taken arms after a call exits the process | 22, 24 | [Report](evidence/batch3-stable/24_exit_counter_underflow.txt) | [Report](evidence/batch3-nightly/24_exit_counter_underflow.txt) |
| [30: panicking destructor](failing/30_panicking_drop/src/main.rs) | Statement after a panicking destructor | 21 | [Report](evidence/batch3-stable/30_panicking_drop.txt) | [Report](evidence/batch3-nightly/30_panicking_drop.txt) |
| [38: destructuring assignment](failing/38_destructuring_assignment/src/main.rs) | Statement after a later assignment target panics | 11 | [Report](evidence/batch4-stable/38_destructuring_assignment.txt) | [Report](evidence/batch4-nightly/38_destructuring_assignment.txt) |
| [39: overloaded addition](failing/39_overloaded_add_unwind/src/main.rs) | Statement and result after overloaded `+` panics | 22, 23 | [Report](evidence/batch4-stable/39_overloaded_add_unwind.txt) | [Report](evidence/batch4-nightly/39_overloaded_add_unwind.txt) |
| [40: unwinding condition](failing/40_unwinding_condition/src/main.rs) | `else` body when the condition panics before selecting either arm | 19 | [Report](evidence/batch4-stable/40_unwinding_condition.txt) | [Report](evidence/batch4-nightly/40_unwinding_condition.txt) |
| [45: await unwind](failing/45_await_unwind/src/main.rs) | Continuation after an awaited future panics | 18, 19 | [Report](evidence/batch4-stable/45_await_unwind.txt) | [Report](evidence/batch4-nightly/45_await_unwind.txt) |
| [46: division unwind](failing/46_division_unwind/src/main.rs) | Continuation after division by zero | 9, 10 | [Report](evidence/batch4-stable/46_division_unwind.txt) | [Report](evidence/batch4-nightly/46_division_unwind.txt) |
| [49: overloaded indexing](failing/49_index_trait_unwind/src/main.rs) | Continuation after overloaded indexing panics | 26, 27 | [Report](evidence/batch4-stable/49_index_trait_unwind.txt) | [Report](evidence/batch4-nightly/49_index_trait_unwind.txt) |
| [50: formatting unwind](failing/50_format_display_unwind/src/main.rs) | Statement after `Display::fmt` panics during formatting | 22 | [Report](evidence/batch4-stable/50_format_display_unwind.txt) | [Report](evidence/batch4-nightly/50_format_display_unwind.txt) |

## Case 24 and process exit

Case 24 calls `std::process::exit(0)`, matching the README's explicit expected-failure
condition. It is included here because its saved coverage marks unexecuted code
as covered. Line 22 exports `9223372036854775807` in JSON, rendered as `18.4E` in
LLVM's text report. Line 24 reports 1. Both should count 0.

The run produced coverage data; its discrepancy is incorrect counts in that
data. The driver asserts 3 calls before exiting, and both corresponding probes
correctly report 3.

## Scope

The other 10 samples catch their intentional panics, complete their runtime
assertions, and exit successfully. Their side-effect counters prove the listed
continuations did not execute. These are related reproducers, without a claim
of 11 independent compiler bugs.

Case 44 is excluded: its continuation executes once but reports 2. That is an
incorrect execution count, but does not mark an unexecuted expression as covered.
Cases that report executed code as uncovered or omit coverage mappings are also
outside this list.

These findings concern the selected expressions and inputs recorded in
[experiments.json](experiments.json). They are not an exhaustive assessment of
every source line. The recorded compilers are Rust 1.98.1 (`48a229cea`, LLVM
22.1.8) and Rust 1.100.0-nightly (`f7d782a3b`, LLVM 23.1.0).
