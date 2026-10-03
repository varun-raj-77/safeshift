//! Toolchain bootstrap only: this is not migration or equivalence evidence.

use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct TemporaryDirectory(PathBuf);

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        if let Err(error) = fs::remove_dir_all(&self.0) {
            eprintln!("Could not clean bootstrap directory {:?}: {error}", self.0);
        }
    }
}

#[cfg(windows)]
fn compile_fixture(fixture: &Path, executable: &Path) -> io::Result<Output> {
    use std::os::windows::process::CommandExt;

    // SS-001's verified installation locations, not a compiler discovery system.
    let developer_environment = Path::new(
        r"C:\Program Files (x86)\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat",
    );
    let compiler = Path::new(r"C:\Program Files\LLVM\bin\clang.exe");
    for required in [developer_environment, compiler] {
        if !required.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Required Windows bootstrap tool is missing: {}",
                    required.display()
                ),
            ));
        }
    }

    eprintln!("C bootstrap compiler: {}", compiler.display());
    // Quoted environment variables keep paths with spaces out of command syntax.
    // /d disables cmd AutoRun; /v:off preserves literal exclamation marks in paths.
    Command::new("cmd.exe")
        .raw_arg(
            r#"/d /v:off /s /c "call "%SS001_VSDEVCMD%" -no_logo -arch=x64 -host_arch=x64 && "%SS001_CLANG%" -std=c11 -Wall -Wextra -Werror "%SS001_FIXTURE%" -o "%SS001_EXECUTABLE%""#,
        )
        .env("SS001_VSDEVCMD", developer_environment)
        .env("SS001_CLANG", compiler)
        .env("SS001_FIXTURE", fixture)
        .env("SS001_EXECUTABLE", executable)
        .current_dir(executable.parent().expect("executable has a directory"))
        .output()
}

#[cfg(not(windows))]
fn compile_fixture(fixture: &Path, executable: &Path) -> io::Result<Output> {
    // CC is a single executable name or path, not a shell command with flags.
    let explicit_compiler = env::var_os("CC");
    let compilers = match &explicit_compiler {
        Some(compiler) => vec![compiler.clone()],
        None => vec!["clang".into(), "gcc".into()],
    };
    for compiler in compilers {
        let result = Command::new(&compiler)
            .args(["-std=c11", "-Wall", "-Wextra", "-Werror"])
            .arg(fixture)
            .arg("-o")
            .arg(executable)
            .current_dir(executable.parent().expect("executable has a directory"))
            .output();
        match result {
            Err(error)
                if explicit_compiler.is_none() && error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(io::Error::new(
                    error.kind(),
                    format!("Could not start C compiler {compiler:?}: {error}"),
                ));
            }
            Ok(output) => {
                eprintln!("C bootstrap compiler: {compiler:?}");
                return Ok(output);
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "No C compiler found: set CC to an executable, or install clang or gcc",
    ))
}

#[test]
fn c_bootstrap_compiles_links_and_runs() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/bootstrap.c");
    assert!(
        fixture.is_file(),
        "Missing C fixture: {}",
        fixture.display()
    );

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after the Unix epoch")
        .as_nanos();
    let directory = env::temp_dir().join(format!(
        "safeshift-ss001-{}-{timestamp}",
        std::process::id()
    ));
    // Atomic creation refuses to reuse any existing directory.
    fs::create_dir(&directory).expect("could not create unique bootstrap directory");
    let directory = TemporaryDirectory(directory);
    let executable = directory
        .0
        .join(format!("bootstrap{}", env::consts::EXE_SUFFIX));

    let compiled = compile_fixture(&fixture, &executable)
        .expect("could not launch C bootstrap compilation/linking");
    assert!(
        compiled.status.success(),
        "C compilation/linking failed ({}).\nstdout:\n{}\nstderr:\n{}",
        compiled.status,
        String::from_utf8_lossy(&compiled.stdout),
        String::from_utf8_lossy(&compiled.stderr),
    );
    eprintln!("C compile/link: {}", compiled.status);

    let executed = Command::new(&executable)
        .current_dir(&directory.0)
        .output()
        .expect("could not execute the compiled C bootstrap program");
    assert!(
        executed.status.success(),
        "C execution failed ({}).\nstdout:\n{}\nstderr:\n{}",
        executed.status,
        String::from_utf8_lossy(&executed.stdout),
        String::from_utf8_lossy(&executed.stderr),
    );
    assert!(
        executed.stdout == b"safeshift-ss001-ok\n" || executed.stdout == b"safeshift-ss001-ok\r\n",
        "Unexpected C stdout: {:?}",
        String::from_utf8_lossy(&executed.stdout),
    );
    eprintln!(
        "C execution: {}; stdout: {:?}",
        executed.status,
        String::from_utf8_lossy(&executed.stdout),
    );
}
