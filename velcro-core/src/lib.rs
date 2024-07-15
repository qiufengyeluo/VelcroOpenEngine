#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]


pub mod parallel;



#[cfg(test)]
mod tests {
    use crate::parallel::spin_mutex;

    use super::*;

    

    #[test]
    fn it_work_parallel() {
        let sp = spin_mutex::SpinMutex::new(false);
        sp.lock();
        sp.unlock();
    }
}