# Coverage counters behind the reports

The symbolic mappings explain more than the evaluated counts in the HTML reports.
All 21 failing cases have raw profiles and mapping dumps from both installed
compilers. The parser is the local `../../../llvm-profparser` checkout, used by
[`coverage-debug`](../../scripts/coverage-debug/src/main.rs).

## Findings

Case 13 maps the unexecuted `*after += 1` to `work`'s counter `c0`, whose value is
1. There is no separate counter or expression for successful return from the
panicking call. The [disassembly](stable/13_unwind_continuation.asm) confirms the
coverage increment occurs at `0x18c3f`, before the call to `may_panic` at
`0x18c95`. The later source increment has no additional coverage increment.
This establishes why the continuation inherits a positive count in this binary.

Case 24 assigns the untaken printing arm `c2 - c0`. The raw counter array for
`run` is `[1, 3, 0]`, so the expression is `0 - 1 = -1`. LLVM exports this region
as `9223372036854775807`. The
[disassembly](stable/24_exit_counter_underflow.asm) shows the first counter
increment at function entry and the third on the `Err` path after `step` returns.
The process exits inside `step` before taking that path. The subtraction is
therefore invalid for this incomplete execution; there is no enormous raw count
in the profile.

Cases 18 and 35 both report an executed guard as uncovered, but encode it
differently. Case 18 uses `c0 - c3 = 1 - 1`; case 35 directly uses `c3 = 0`.
Their similar HTML symptoms do not establish an identical mapping defect.

Cases 21 and 22 assign one function-entry count to an expression that executes
twice. Case 33 uses `c0 - c1 = 1 - 0` for its twice-evaluated macro argument.
The source mappings do not represent the asserted evaluation multiplicity.

Cases 01, 25, 26 and 27 have no code region covering the failing source anchors.
This differs from a mapped region with a zero counter. The dumps do not establish
which compiler transformation lost those mappings.

## Every failing case

`cN[value]` denotes raw counter N within the containing function; counter IDs are
not global. Arithmetic below uses signed integers to expose negative results.
The table describes the mismatching anchors, rather than every region.
Both compilers have the same formulas and values at these anchors.

| Case | Region or expression behind the mismatch | Dumps |
| --- | --- | --- |
| [01_tracing_async](../../failing/01_tracing_async/src/main.rs) | line 17 `db_get(id).await?`: no code region; line 18 `Ok(row)`: no code region | [stable](stable/01_tracing_async.json), [nightly](nightly/01_tracing_async.json) |
| [13_unwind_continuation](../../failing/13_unwind_continuation/src/main.rs) | line 11 `*after += 1`: `c0[1]` | [stable](stable/13_unwind_continuation.json), [nightly](nightly/13_unwind_continuation.json) |
| [18_or_pattern_guard](../../failing/18_or_pattern_guard/src/main.rs) | line 5 `*checks += 1`: `(c0[1] - c3[1])`; line 6 `value > 4`: `(c0[1] - c3[1])` | [stable](stable/18_or_pattern_guard.json), [nightly](nightly/18_or_pattern_guard.json) |
| [21_repeated_macro_argument](../../failing/21_repeated_macro_argument/src/main.rs) | line 9 `*hits += 1`: `c0[1]` | [stable](stable/21_repeated_macro_argument.json), [nightly](nightly/21_repeated_macro_argument.json) |
| [22_overlapping_or_guard](../../failing/22_overlapping_or_guard/src/main.rs) | line 5 `*checks += 1`: `c0[1]`; line 6 `false`: `c0[1]` | [stable](stable/22_overlapping_or_guard.json), [nightly](nightly/22_overlapping_or_guard.json) |
| [23_bounds_partial_tuple](../../failing/23_bounds_partial_tuple/src/main.rs) | line 11 `*after += 1`: `c0[1]` | [stable](stable/23_bounds_partial_tuple.json), [nightly](nightly/23_bounds_partial_tuple.json) |
| [24_exit_counter_underflow](../../failing/24_exit_counter_underflow/src/main.rs) | line 20 `println!("{code}")`: `(c2[0] - c0[1])`; line 22 `Err(code),`: `c0[1]` | [stable](stable/24_exit_counter_underflow.json), [nightly](nightly/24_exit_counter_underflow.json) |
| [25_derive_expression](../../failing/25_derive_expression/src/main.rs) | line 4 `*.0`: no code region; line 4 `"selected"`: no code region; line 4 `"untaken"`: no code region | [stable](stable/25_derive_expression.json), [nightly](nightly/25_derive_expression.json) |
| [26_async_trait_macro](../../failing/26_async_trait_macro/src/main.rs) | line 15 `*self += 1`: no code region; line 16 `ok {`: no code region; line 16 `Ok(4)`: no code region; line 16 `Err(())`: no code region; line 17 `Ok(value + 1)`: no code region | [stable](stable/26_async_trait_macro.json), [nightly](nightly/26_async_trait_macro.json) |
| [27_thread_local_initializer](../../failing/27_thread_local_initializer/src/main.rs) | line 7 `INITIALIZATIONS.fetch_add`: no code region; line 8 `41`: no code region | [stable](stable/27_thread_local_initializer.json), [nightly](nightly/27_thread_local_initializer.json) |
| [30_panicking_drop](../../failing/30_panicking_drop/src/main.rs) | line 18 `*after += 1`: `c0[1]` | [stable](stable/30_panicking_drop.json), [nightly](nightly/30_panicking_drop.json) |
| [33_macro_short_circuit](../../failing/33_macro_short_circuit/src/main.rs) | line 9 `*calls += 1`: `(c0[1] - c1[0])`; line 10 `*calls == 2`: `(c0[1] - c1[0])` | [stable](stable/33_macro_short_circuit.json), [nightly](nightly/33_macro_short_circuit.json) |
| [35_matches_guard](../../failing/35_matches_guard/src/main.rs) | line 3 `*checks += 1`: `c3[0]`; line 4 `value > 4`: `c3[0]` | [stable](stable/35_matches_guard.json), [nightly](nightly/35_matches_guard.json) |
| [38_destructuring_assignment](../../failing/38_destructuring_assignment/src/main.rs) | line 8 `*after += 1`: `c0[1]` | [stable](stable/38_destructuring_assignment.json), [nightly](nightly/38_destructuring_assignment.json) |
| [39_overloaded_add_unwind](../../failing/39_overloaded_add_unwind/src/main.rs) | line 21 `*after += 1`: `c0[1]`; line 22 `total + 1`: `c0[1]` | [stable](stable/39_overloaded_add_unwind.json), [nightly](nightly/39_overloaded_add_unwind.json) |
| [40_unwinding_condition](../../failing/40_unwinding_condition/src/main.rs) | line 16 `*no += 1`: `(c0[1] - c2[0])` | [stable](stable/40_unwinding_condition.json), [nightly](nightly/40_unwinding_condition.json) |
| [44_once_lock_retry](../../failing/44_once_lock_retry/src/main.rs) | line 14 `*after += 1`: `c0[2]`; line 15 `*value`: `c0[2]` | [stable](stable/44_once_lock_retry.json), [nightly](nightly/44_once_lock_retry.json) |
| [45_await_unwind](../../failing/45_await_unwind/src/main.rs) | line 17 `*after += 1`: `(c1[1] - c2[0])`; line 18 `value + 1`: `(c1[1] - c2[0])` | [stable](stable/45_await_unwind.json), [nightly](nightly/45_await_unwind.json) |
| [46_division_unwind](../../failing/46_division_unwind/src/main.rs) | line 6 `*after += 1`: `c0[1]`; line 7 `value + 1`: `c0[1]` | [stable](stable/46_division_unwind.json), [nightly](nightly/46_division_unwind.json) |
| [49_index_trait_unwind](../../failing/49_index_trait_unwind/src/main.rs) | line 26 `after.fetch_add`: `c0[1]`; line 27 `value + 1`: `c0[1]` | [stable](stable/49_index_trait_unwind.json), [nightly](nightly/49_index_trait_unwind.json) |
| [50_format_display_unwind](../../failing/50_format_display_unwind/src/main.rs) | line 22 `after.fetch_add`: `c0[1]` | [stable](stable/50_format_display_unwind.json), [nightly](nightly/50_format_display_unwind.json) |

## Reproduce the capture

From the repository root, with the sibling `../llvm-profparser` checkout present:

```sh
python3 scripts/capture-debug.py
python3 scripts/capture-debug.py --toolchain nightly
python3 scripts/capture-debug.py 13_unwind_continuation 24_exit_counter_underflow
```

The script builds each instrumented sample, then runs the finished binary with a
separate `LLVM_PROFILE_FILE`. This excludes profiles emitted by build-time
procedural macros. It retains the matching binaries under
`target/coverage-debug/<toolchain>/<case>/`; these binaries are not committed.
The raw `.profraw` files and JSON dumps are retained here. Run samples without
additional arguments, as required by their conditional-panic drivers.

To inspect an existing profile and matching binary directly:

```sh
cargo run --locked --manifest-path scripts/coverage-debug/Cargo.toml -- \
  evidence/coverage-debug/stable/13_unwind_continuation.profraw \
  target/coverage-debug/stable/13_unwind_continuation/sample_13_unwind_continuation \
  failing/13_unwind_continuation/src/main.rs
```

The sibling CLI can also display raw counts:

```sh
cargo run --manifest-path ../llvm-profparser/Cargo.toml --bin profparser -- \
  show --input evidence/coverage-debug/stable/13_unwind_continuation.profraw \
  --all-functions --counts
```

The inspector exposes function names, name and structural hashes, raw counter
arrays, expression operands, region kinds, source spans and secondary counters.
It checks structural hashes when a profile record exists. Missing profile records
remain `null`; they are not silently converted to zero. Source coordinates use
LLVM's one-based byte columns and half-open ranges.

Profiles alone do not contain the source regions or expression graph. Those
come from the matching executable's coverage sections, as described in the
[LLVM coverage mapping format](https://llvm.org/docs/CoverageMappingFormat.html).
These expressions are arithmetic over coverage counters, not Rust AST expressions.
The saved disassembly additionally uses the executable's debug line information.

## Validation and scope

The captures use Rust 1.98.1 / LLVM 22.1.8 and Rust 1.100.0-nightly / LLVM 23.1.0.
Each JSON file records the exact compiler, parser revision, parser working-tree
status, commands and hashes of its source, raw profile, binary and lockfiles.

Each retained raw profile was merged independently with the matching toolchain's
`llvm-profdata`, then exported using `llvm-cov` and its saved binary. All 509
regions per compiler matched the parser's spans and evaluated counter arithmetic,
except that case 24's signed result of -1 is exported as the large value described
above. All matched function structural hashes agreed. Runtime assertions passed.
The inspector keeps arithmetic symbolic rather than reproducing LLVM's unsigned
conversion and export behaviour.

These observations localise problems to particular mappings and counter formulas.
They do not by themselves identify the responsible rustc pass. MIR coverage dumps
would be the next step for determining why those mappings were chosen.
