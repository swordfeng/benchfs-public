//! Minimal executable wiring smoke for the NullFS scaffold.

use benchfs_fuse::{Adapter, MountSource, RuntimeConfig, run_mount};
use benchfs_nullfs::NullFs;
use benchfs_sdk::{Errno, FilesystemOperations, RequestContext};
use std::sync::Arc;

async fn smoke() -> Result<(), String> {
    let adapter = Adapter::new(Arc::new(NullFs));
    let context = RequestContext {
        uid: 0,
        gid: 0,
        pid: 1,
        umask: 0o022,
    };
    let error = adapter
        .lookup(&context, 1, b"smoke")
        .await
        .expect_err("NullFS must not implement lookup");
    if error.errno() != Errno::NotSupported {
        return Err(format!("unexpected NullFS errno: {error}"));
    }
    println!("benchfs-nullfs smoke: EOPNOTSUPP");
    Ok(())
}

fn main() {
    let mut arguments = std::env::args_os();
    let _program = arguments.next();
    match (arguments.next(), arguments.next()) {
        (Some(flag), None) if flag == "--smoke" => {
            if let Err(error) = futures::executor::block_on(smoke()) {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
        (Some(flag), None) if flag == "--help" || flag == "-h" => {
            println!("Usage: benchfs-nullfs --smoke | <mountpoint>");
        }
        (Some(mountpoint), None) => {
            let config = match RuntimeConfig::from_env() {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("benchfs-nullfs invalid runtime configuration: {error}");
                    std::process::exit(error.raw_os_error());
                }
            };
            // Daemon contract: BENCHFS_SNAPSHOT selects an independent
            // read-only snapshot FUSE mount; without it the current view is
            // served. NullFS returns EOPNOTSUPP for both sources.
            let source = match MountSource::from_env() {
                Ok(source) => source.unwrap_or(MountSource::Current),
                Err(error) => {
                    eprintln!("benchfs-nullfs invalid BENCHFS_SNAPSHOT: {error}");
                    std::process::exit(error.raw_os_error());
                }
            };
            let filesystem: Arc<dyn FilesystemOperations> = Arc::new(NullFs);
            if let Err(error) = run_mount(filesystem, std::path::Path::new(&mountpoint), config) {
                eprintln!("benchfs-nullfs mount failed: {error}");
                std::process::exit(error.raw_os_error());
            }
            let _ = source; // NullFS never reaches a real mount; documented for wiring parity.
        }
        _ => {
            eprintln!("Usage: benchfs-nullfs --smoke | <mountpoint>");
            std::process::exit(22);
        }
    }
}
