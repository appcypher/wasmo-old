#[cfg(unix)]
#[path = "unix.rs"]
mod unix;

#[cfg(windows)]
#[path = "win32.rs"]
mod win32;


#[cfg(unix)]
pub use unix::MemoryGenerator;

#[cfg(windows)]
pub use win32::MemoryGenerator;

