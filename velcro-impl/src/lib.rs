//! 3D/2D/Network Game Engine.

#![allow(clippy::too_many_arguments)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(clippy::approx_constant)]


#[doc(inline)]
pub use velcro_core as core;

#[macro_export]
macro_rules! define_with {
    ($(#[$attr:meta])* fn $name:ident($field:ident: $ty:ty)) => {
        $(#[$attr])*
        pub fn $name(mut self, value: $ty) -> Self {
            self.$field = value;
            self
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn test_assembly() {
        
    }
}