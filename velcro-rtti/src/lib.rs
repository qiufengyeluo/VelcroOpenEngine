mod type_traits;
mod reflect;
mod sstorage;
mod memory;

#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate lazy_static;

pub use num_traits;
pub use parking_lot;

use velcro_utils::UUID;
use velcro_utils::hasder::*;

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {

    }
}
