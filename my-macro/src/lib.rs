use proc_macro::TokenStream;

mod elapsed;

/// A proc macro for calculating the elapsed time of the function
#[proc_macro_attribute]
#[cfg(not(test))]
pub fn elapsed(args: TokenStream, func: TokenStream) -> TokenStream {
    elapsed::elapsed(args, func)
}

#[cfg(test)]
mod test {
    use my_macro::elapsed;
    use std::thread;
    use std::time::Duration;

    fn demo(t: u64) {
        let secs = Duration::from_secs(t);
        thread::sleep(secs);
    }

    #[test]
    fn test_my_macro() {
        demo(4);
        demo(2);
    }
}