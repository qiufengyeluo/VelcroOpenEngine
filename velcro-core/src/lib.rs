#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]

#[macro_use]
extern crate memoffset;
#[macro_use]
extern crate lazy_static;
extern crate core;

use velcro_utils::UUID;
//use velcro_derive::*;

pub use num_traits;
pub use parking_lot;
/*pub mod reflect;*/

mod math;
mod parallel;
mod interface;


pub use math::random::*;




/*mod type_traits;
mod sstorage;
mod variable;
mod serialization;



mod visitor;

pub use type_traits::prelude::*;
*/








#[cfg(test)]
mod tests {
    use crate::parallel::spin_mutex;

    use super::*;

  

    #[test]
    fn it_work_random() {
        println!("random: {}", math::random::get_random::<i32>().unwrap());
        let mut sft = math::sfmt::Sfmt::new();
        println!("sfmt: {}", sft.rand_r32());
    }

    #[test]
    fn it_work_parallel() {
        let sp = spin_mutex::SpinMutex::new(false);
        sp.lock();
        sp.unlock();
    }
}