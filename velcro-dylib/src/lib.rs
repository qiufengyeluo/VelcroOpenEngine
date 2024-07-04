//! A crate that allows using VelcroOpen as a dynamically linked library. It could be useful for fast
//! prototyping, that can save some time on avoiding potentially time-consuming static linking
//! stage.
//!
//! The crate just re-exports everything from the engine, and you can use it as VelcroOpen. To use the
//! crate all you need to do is re-define `velcro` dependency in your project like so:
//!
//! ```toml
//! [dependencies.velcro]
//! version = "0.1.0"
//! registry = "velcro-dylib"
//! package = "velcro-dylib"
//! ```
//!
//! You can also use the latest version from git:
//!
//! ```toml
//! [dependencies.velcro]
//! git = "https://github.com/yamakiller/velcro-open"
//! package = "velcro-dylib"
//! ```

// Just re-export everything.
pub use velcro_impl::*;