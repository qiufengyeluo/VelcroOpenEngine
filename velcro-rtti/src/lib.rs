mod type_traits;
mod reflect;
mod reflect_context;
mod sstorage;
mod memory;

mod variable;

#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate lazy_static;

pub use num_traits;
pub use parking_lot;

use velcro_utils::{UUID, hasder::*};

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {

    }
}
