use std::ffi::OsStr;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use anyhow::Context;
use directories::UserDirs;
use nix::time::{clock_gettime, ClockId};
use smithay::utils::{Logical, Point, Rectangle};
use time::OffsetDateTime;

pub fn get_monotonic_time() -> Duration {
    Duration::from(clock_gettime(ClockId::CLOCK_MONOTONIC).unwrap())
}

pub fn center(rect: Rectangle<i32, Logical>) -> Point<i32, Logical> {
    rect.loc + rect.size.downscale(2).to_point()
}

pub fn make_screenshot_path() -> anyhow::Result<PathBuf> {
    let dirs = UserDirs::new().context("error retrieving home directory")?;
    let mut path = dirs.picture_dir().map(|p| p.to_owned()).unwrap_or_else(|| {
        let mut dir = dirs.home_dir().to_owned();
        dir.push("Pictures");
        dir
    });
    path.push("Screenshots");

    unsafe {
        // are you kidding me
        time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Unsound);
    };

    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let desc = time::macros::format_description!(
        "Screenshot from [year]-[month]-[day] [hour]-[minute]-[second].png"
    );
    let name = now.format(desc).context("error formatting time")?;
    path.push(name);

    Ok(path)
}

/// Spawns the command to run independently of the compositor.
pub fn spawn(command: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let _span = tracy_client::span!();

    let command = command.as_ref();

    let mut process = Command::new(command);
    process
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Double-fork to avoid having to waitpid the child.
    unsafe {
        process.pre_exec(|| {
            match libc::fork() {
                -1 => return Err(io::Error::last_os_error()),
                0 => (),
                _ => libc::_exit(0),
            }

            Ok(())
        });
    }

    let mut child = match process.spawn() {
        Ok(child) => child,
        Err(err) => {
            warn!("error spawning {command:?}: {err:?}");
            return;
        }
    };

    match child.wait() {
        Ok(status) => {
            if !status.success() {
                warn!("child did not exit successfully: {status:?}");
            }
        }
        Err(err) => {
            warn!("error waiting for child: {err:?}");
        }
    }
}
