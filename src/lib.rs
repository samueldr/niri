#[macro_use]
extern crate tracing;

pub mod animation;
pub mod backend;
pub mod cli;
pub mod cursor;
#[cfg(feature = "dbus")]
pub mod dbus;
pub mod frame_clock;
pub mod handlers;
pub mod input;
pub mod ipc;
pub mod layout;
pub mod niri;
pub mod protocols;
pub mod render_helpers;
pub mod ui;
pub mod utils;

#[cfg(not(feature = "xdp-gnome-screencast"))]
pub mod dummy_pw_utils;
#[cfg(feature = "xdp-gnome-screencast")]
pub mod pw_utils;

#[cfg(not(feature = "xdp-gnome-screencast"))]
pub use dummy_pw_utils as pw_utils;
