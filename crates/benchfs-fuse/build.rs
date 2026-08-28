//! Builds the fixed libfuse 3.18.2 C ABI bridge on Linux targets.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(benchfs_skip_native)");
    println!("cargo:rerun-if-changed=native/benchfs_bridge.h");
    println!("cargo:rerun-if-changed=native/benchfs_fuse.c");
    println!("cargo:rerun-if-env-changed=BENCHFS_FUSE_CHECK_NATIVE");
    println!("cargo:rerun-if-env-changed=BENCHFS_FUSE_SKIP_NATIVE");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        return;
    }
    if env::var_os("BENCHFS_FUSE_SKIP_NATIVE").is_some() {
        println!("cargo:rustc-cfg=benchfs_skip_native");
        return;
    }

    // Used by Linux CI when the pinned libfuse development headers are not
    // installed: compile the Rust native module with cargo check, but do not
    // build or link the C bridge. Production builds never set this switch.
    if env::var_os("BENCHFS_FUSE_CHECK_NATIVE").is_some() {
        return;
    }

    let version = command_output("pkg-config", &["--modversion", "fuse3"]);
    assert_eq!(
        version.trim(),
        "3.18.2",
        "BenchFS requires exactly libfuse 3.18.2"
    );
    let cflags = command_output("pkg-config", &["--cflags", "fuse3"]);
    let libs = command_output("pkg-config", &["--libs", "fuse3"]);
    let output = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is set by Cargo"));
    let object = output.join("benchfs_fuse.o");
    let archive = output.join("libbenchfs_fuse_native.a");

    let mut compiler = Command::new(env::var_os("CC").unwrap_or_else(|| "cc".into()));
    compiler
        .arg("-std=c11")
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Werror")
        .arg("-fPIC")
        .arg("-DFUSE_USE_VERSION=35")
        .arg("-c")
        .arg("native/benchfs_fuse.c")
        .arg("-o")
        .arg(&object);
    for flag in cflags.split_whitespace() {
        compiler.arg(flag);
    }
    run(&mut compiler, "compile native libfuse bridge");

    let mut archiver = Command::new(env::var_os("AR").unwrap_or_else(|| "ar".into()));
    archiver.arg("crs").arg(&archive).arg(&object);
    run(&mut archiver, "archive native libfuse bridge");

    println!("cargo:rustc-link-search=native={}", output.display());
    println!("cargo:rustc-link-lib=static=benchfs_fuse_native");
    for flag in libs.split_whitespace() {
        if let Some(path) = flag.strip_prefix("-L") {
            println!("cargo:rustc-link-search=native={path}");
        } else if let Some(library) = flag.strip_prefix("-l") {
            println!("cargo:rustc-link-lib={library}");
        }
    }
}

fn command_output(program: &str, arguments: &[&str]) -> String {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("failed to run {program}: {error}"));
    assert!(
        output.status.success(),
        "{program} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("tool output is UTF-8")
}

fn run(command: &mut Command, description: &str) {
    let status = command
        .status()
        .unwrap_or_else(|error| panic!("failed to {description}: {error}"));
    assert!(status.success(), "failed to {description}");
}
