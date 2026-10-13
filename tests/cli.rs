//! End-to-end tests driving the compiled binary.

use std::process::Command;

fn run(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_rfetch"))
        .args(args)
        .output()
        .expect("rfetch binary should be runnable");
    (
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
        out.status.success(),
    )
}

#[test]
fn default_run_succeeds() {
    let (stdout, stderr, ok) = run(&[]);
    assert!(ok, "stderr: {stderr}");
    assert!(!stdout.is_empty());
}

#[test]
fn pipe_output_has_no_ansi_escapes() {
    let (stdout, _, ok) = run(&["--pipe"]);
    assert!(ok);
    assert!(!stdout.contains('\x1b'), "pipe output must not be coloured");
}

#[test]
fn pipe_output_uses_key_colon_value() {
    let (stdout, _, _) = run(&["--pipe", "--modules", "os,ram"]);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 2, "got {stdout}");
    assert!(lines[0].starts_with("OS: "), "got {}", lines[0]);
    assert!(lines[1].starts_with("RAM: "), "got {}", lines[1]);
}

#[test]
fn json_output_is_a_single_object() {
    let (stdout, _, ok) = run(&["--json", "--modules", "os"]);
    assert!(ok);
    let line = stdout.trim();
    assert!(line.starts_with('{') && line.ends_with('}'), "got {line}");
    assert!(line.contains("\"os\":"), "got {line}");
    assert!(!line.contains('\x1b'));
}

#[test]
fn hide_modules_removes_lines() {
    let (all, _, _) = run(&["--pipe"]);
    let (hidden, _, _) = run(&["--pipe", "--hide-modules", "gpu,disk,cpu"]);
    assert!(all.lines().count() > hidden.lines().count());
    assert!(!hidden.contains("GPU:"));
    assert!(!hidden.contains("CPU:"));
}

#[test]
fn modules_order_is_respected() {
    let (stdout, _, _) = run(&["--pipe", "--modules", "ram,os"]);
    let keys: Vec<&str> = stdout
        .lines()
        .map(|l| l.split(':').next().unwrap())
        .collect();
    assert_eq!(keys, vec!["RAM", "OS"]);
}

#[test]
fn logo_off_produces_no_art() {
    let (with_logo, _, _) = run(&["--logo", "off", "--modules", "os"]);
    for line in with_logo.lines() {
        assert!(line.starts_with("OS"), "art leaked into output: {line:?}");
    }
}

#[test]
fn explicit_logo_is_used() {
    let (stdout, _, ok) = run(&["--logo", "arch", "--modules", "os"]);
    assert!(ok);
    assert!(stdout.contains("OS"), "got {stdout}");
    // The Arch art's first line is a lone backtick.
    assert!(stdout.contains('`'), "arch art missing: {stdout}");
}

#[test]
fn no_color_flag_emits_plain_text() {
    let (stdout, _, _) = run(&["--no-color"]);
    assert!(!stdout.contains('\x1b'), "got {stdout:?}");
}

#[test]
fn piped_output_is_never_coloured() {
    // stdout is a pipe under test, so colours must be off regardless.
    let (stdout, _, _) = run(&[]);
    assert!(!stdout.contains('\x1b'));
}

#[test]
fn unknown_module_exits_with_error() {
    let (_, stderr, ok) = run(&["--modules", "nonexistent"]);
    assert!(!ok, "should exit non-zero");
    assert!(!stderr.is_empty());
}

#[test]
fn version_flag_prints_version() {
    let (stdout, _, ok) = run(&["--version"]);
    assert!(ok);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn help_lists_every_module() {
    let (stdout, _, ok) = run(&["--help"]);
    assert!(ok);
    for m in [
        "os", "host", "kernel", "uptime", "cpu", "gpu", "ram", "disk",
    ] {
        assert!(stdout.contains(m), "{m} missing from --help");
    }
}

#[test]
fn duplicate_modules_are_rendered_once() {
    let (stdout, _, _) = run(&["--pipe", "--modules", "ram,ram,ram"]);
    assert_eq!(stdout.lines().count(), 1, "got {stdout}");
}

#[test]
fn nerd_separator_replaces_colon() {
    let (stdout, _, _) = run(&["--pipe", "--nerd-separator", "--modules", "os"]);
    assert!(!stdout.contains("OS: "), "got {stdout}");
    assert!(stdout.contains('\u{f420}'), "got {stdout:?}");
}
