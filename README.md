# SafeShift

**SS-001 bootstrap implementation and exercised CI are complete; external
adversarial review remains pending.**

Local checks on the documented Windows environment and GitHub-hosted Ubuntu 24.04
CI have passed. In both exercised environments, the C fixture compiled, linked,
executed, and produced the expected output. This is bootstrap/toolchain evidence
only, not migration, equivalence, or behavior-preservation evidence. Claude external
adversarial review is still pending.

## Research purpose

SafeShift investigates whether important memory-unsafe legacy C components can be
replaced incrementally with Rust while establishing honest, reproducible evidence
about required behavior preservation and maintaining that evidence as assumptions
change. The research question is whether this can materially improve the total
cost and assurance of migration and maintenance, not whether Rust can be generated.

The project exists only if it demonstrates a meaningful advantage over the strongest
practical existing migration workflows. No such advantage has been demonstrated.

## Current implementation

- One root Rust binary package and workspace named `safeshift`, using edition 2024
  and workspace resolver 3.
- Cargo's test harness, with one trivial Rust test of a standard-library string
  operation and one integration test that compiles, links, executes, and checks a
  single tiny C fixture.
- Zero third-party Rust dependencies.

The executable only prints `SafeShift bootstrap only.` The C fixture and its test
validate the toolchain/harness path. Successful execution is ordinary test evidence
for that bootstrap path, not migration or research evidence.

## Explicit non-claims

The current implementation does not establish that SafeShift can:

- Translate real C to Rust or produce a safe Rust replacement.
- Preserve real component behavior or establish C/Rust equivalence.
- Formally verify a migration, infer behavior contracts, or analyze migration
  boundaries.
- Beat C2Rust, SAW/Crux, Kani, CBMC, VERT, &inator, CRISP & CLEAR, manual migration
  workflows, or any other baseline.
- Reduce migration effort or maintain migration evidence across source changes.

The fixture is neither a real migration target nor a comparison with a Rust
replacement. Future evidence must distinguish PROVED, BOUNDED, TESTED, REFUTED,
UNSUPPORTED, and ASSUMED. This is project doctrine; the taxonomy is not implemented
in SS-001.

## Prerequisites

Use Git and a stable Rust toolchain supporting edition 2024, including `rustc`,
Cargo, rustfmt, and Clippy. Rust commands must be available to the shell running the
commands below. A usable native C compiler, headers, libraries, and linker are also
required. Clang is preferred where available; GCC is acceptable on non-Windows.

On Windows, the current test requires LLVM/Clang and Microsoft native x64 C/C++
build tools with a Windows SDK. It currently uses these fixed installation paths:

```text
C:\Program Files\LLVM\bin\clang.exe
C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat
```

The test loads `VsDevCmd.bat -no_logo -arch=x64 -host_arch=x64` inside a child
`cmd.exe` before invoking Clang. This supplies the Microsoft header/library and
linker environment; locating Clang alone is insufficient. You do not need to start
the test runner from Developer Command Prompt. The test does not persist any
environment changes. Other Windows installation layouts are not automatically
discovered.

On non-Windows, the test honors `CC` when supplied as a single executable name or
path, not a shell command containing arguments. Otherwise it attempts `clang`, then
`gcc` if Clang cannot be found. A compiler invocation or compilation failure is
reported; an explicitly supplied `CC` does not fall back to another compiler.
The non-Windows path has been exercised successfully in GitHub-hosted Ubuntu CI
with `CC=clang`; the GCC fallback was not exercised.

## Reproduction

Run from the repository root:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace --locked
cargo test --workspace --locked
```

The test command runs the trivial Rust harness test and the C bootstrap integration
test: two tests in total.

## CI

`.github/workflows/ci.yml` defines one `ubuntu-24.04` job for pushes and pull
requests. It checks out the repository, selects stable Rust with rustfmt and
Clippy, prints tool versions, and runs the four reproduction commands above.
It uses the runner's installed Clang with `CC=clang`; the existing Rust integration
test performs the real C compilation, linking, and execution. There is no separate
C test in the workflow. [Run 37088601482](https://github.com/varun-raj-77/safeshift/actions/runs/37088601482)
passed on GitHub-hosted Ubuntu 24.04.5 LTS for commit
`1ca8fdfe57d27b27dc7255da6442b7a39783fe66`. Both tests passed, including C
compile/link and execution with exit status 0 and stdout `"safeshift-ss001-ok\n"`.

## C bootstrap sequence

The integration test:

1. Locates `tests/fixtures/bootstrap.c` relative to the Cargo manifest directory.
2. Creates a unique output directory under the operating system's temporary
   directory, refusing to reuse an existing directory.
3. Compiles the fixture with `-std=c11 -Wall -Wextra -Werror`.
4. Links a native executable in the same compiler invocation and checks its status.
5. Executes the resulting program and captures its output.
6. Requires successful process termination.
7. Requires stdout to be exactly `safeshift-ss001-ok` followed by LF or CRLF, with no
   additional stdout.
8. Attempts to remove the temporary directory and its generated files when the
   cleanup guard is dropped, including during ordinary panic unwinding. Cleanup
   errors are printed; cleanup is not guaranteed after forced process termination.

Generated C executables and intermediate artifacts use temporary storage outside
the repository in the tested environment. Cargo's Rust build artifacts go under
the ignored `target/` directory.

## Environment exercised so far

| Component | Observed environment/version |
|---|---|
| Operating system / architecture | Windows x86-64 |
| Rust | 1.99.0 |
| Cargo | 1.99.0 |
| rustfmt | 1.10.0-stable |
| Clippy | 0.1.99 |
| Rust host | `x86_64-pc-windows-msvc` |
| Clang | 23.1.2, targeting `x86_64-pc-windows-msvc` |
| Visual Studio Build Tools | 18.10.2 |
| MSVC | 19.51.36260 |
| Windows SDK | 10.0.26100.0 |
| Git | 2.55.0.windows.1 |

These versions describe the local environment actually exercised. They are not
exact-version requirements or a universal platform guarantee. The fixed Windows
paths above remain a current implementation constraint. A real target build/source
manifest belongs to SS-003, not this bootstrap.

## Repository layout

```text
.
├── .github/
│   └── workflows/
│       └── ci.yml
├── Cargo.toml
├── Cargo.lock
├── .gitignore
├── README.md
├── src/
│   └── main.rs
├── tests/
│   ├── bootstrap.rs
│   └── fixtures/
│       └── bootstrap.c
├── instruction.txt
├── FINAL_PROJECT_SEARCH_2026-10-02.md
├── FINAL_SEARCH_SCORECARDS_2026-10-02.md
└── FINAL_SEARCH_PROBLEM_ATLAS_2026-10-02.md
```

The four root instruction/research documents govern the project; this README
describes the current implementation and how to reproduce its checks.

## Next milestone boundary

SS-001 ends at the bootstrap implementation and CI workflow definition. SS-002 will
reproduce the original build/tests of one real C baseline target and capture the
actual environment needed to run it. No target has been selected, and the current
toy fixture is not SS-002 input.
