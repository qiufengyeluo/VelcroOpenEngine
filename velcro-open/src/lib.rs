#[cfg(not(feature = "dylib"))]
pub use velcro_impl::*;

#[cfg(feature = "dylib")]
pub use velcro_dylib::*;