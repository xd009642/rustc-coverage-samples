"""Run one coverage experiment at a time and retain portable source evidence.

Invoke with python3; this file is intentionally not an executable script.
"""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


ROOT = Path(__file__).resolve().parents[1]


def assess(case, toolchain, output, plain=False):
    package = "sample_" + case.name
    binary = package + ("_plain" if plain else "")
    source_path = case / ("src/bin/plain.rs" if plain else "src/main.rs")
    source = source_path.read_text()
    expectations = json.loads((case / "expectations.json").read_text())
    raw_path = ROOT / "target" / "coverage-exports" / f"{binary}.json"
    raw_path.parent.mkdir(parents=True, exist_ok=True)
    command = [
        "cargo", f"+{toolchain}", "llvm-cov", "run", "--locked",
        "-p", package, "--bin", binary, "--json", "--output-path", str(raw_path),
    ]
    subprocess.run(command, cwd=ROOT, check=True)
    export = json.loads(raw_path.read_text())
    filename = str(source_path)
    files = [
        file for unit in export["data"] for file in unit["files"]
        if file["filename"] == filename
    ]
    assert len(files) == 1, f"Expected one coverage file for {filename}"
    file = files[0]
    functions = [
        function for unit in export["data"] for function in unit["functions"]
        if filename in function["filenames"]
    ]
    probes = []
    # Restrict anchors to the subject, excluding the assertion driver. This
    # keeps a repeated literal in main from selecting the wrong source span.
    subject = source.split("fn main()", 1)[0]
    for probe in expectations["probes"]:
        anchor = probe["source"]
        assert subject.count(anchor) == 1, f"Ambiguous/missing anchor: {anchor!r}"
        offset = subject.index(anchor)
        line = subject[:offset].count("\n") + 1
        column = len(subject[subject.rfind("\n", 0, offset) + 1:offset].encode()) + 1
        position = (line, column)
        preceding = [s for s in file["segments"] if tuple(s[:2]) <= position]
        segment = preceding[-1] if preceding else None
        # LLVM segment intervals are half-open. A segment without a count or
        # marked as a gap does not establish coverage of an executable token.
        actual = segment[2] if segment and segment[3] and not segment[5] else None
        regions = [
            region for function in functions for region in function["regions"]
            if function["filenames"][region[5]] == filename
            and region[7] == 0
            and tuple(region[:2]) <= position < tuple(region[2:4])
        ]
        probes.append({
            **probe, "position": list(position), "actual": actual,
            "matches": actual == probe["count"], "containing_code_regions": regions,
        })
    # Keep LLVM's arrays unchanged, but remove unrelated dependency functions
    # and make filenames portable. Full exports remain under target/.
    file["filename"] = source_path.relative_to(ROOT).as_posix()
    for function in functions:
        function["filenames"] = [
            Path(path).relative_to(ROOT).as_posix()
            if Path(path).is_relative_to(ROOT) else path
            for path in function["filenames"]
        ]
    evidence = {
        "case": case.name, "variant": "plain" if plain else "original",
        "rustc": subprocess.check_output(
            ["rustc", f"+{toolchain}", "--version", "--verbose"], text=True
        ).strip(),
        "cargo_llvm_cov": subprocess.check_output(
            ["cargo", "llvm-cov", "--version"], text=True
        ).strip(),
        "command": command[:-1] + [str(raw_path.relative_to(ROOT))],
        "source_sha256": hashlib.sha256(source_path.read_bytes()).hexdigest(),
        "lock_sha256": hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest(),
        "expectations_sha256": hashlib.sha256((case / "expectations.json").read_bytes()).hexdigest(),
        "llvm_export_version": export["version"],
        "probes": probes, "file": file, "functions": functions,
    }
    destination = output / (case.name + ("_plain" if plain else "") + ".json")
    destination.write_text(json.dumps(evidence, indent=2) + "\n")
    mismatches = sum(not probe["matches"] for probe in probes)
    print(f"{destination.name}: {mismatches}/{len(probes)} probes differ", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("cases", nargs="*", help="Case directory names; default: all")
    parser.add_argument("--toolchain", default="stable")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    available = {
        case.name: case
        for group in ("passing", "failing")
        for case in (ROOT / group).iterdir()
        if case.is_dir()
    }
    unknown = set(args.cases) - available.keys()
    if unknown:
        parser.error(f"Unknown cases: {', '.join(sorted(unknown))}")
    cases = [available[name] for name in (args.cases or sorted(available))]
    for case in cases:
        assess(case, args.toolchain, args.output)
        if (case / "src/bin/plain.rs").exists():
            # The control must differ only by the attribute under investigation.
            assert (case / "src/bin/plain.rs").read_text() == (case / "src/main.rs").read_text().replace("#[tracing::instrument]\n", "")
            assess(case, args.toolchain, args.output, plain=True)


if __name__ == "__main__":
    main()
