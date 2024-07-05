
pub mod sse;
pub mod neon;

#[cfg(any(target_arch = "x86_64", target_arch="x86"))]
pub use sse::*;

#[cfg(target_arch = "arm")]
pub use neon::*;