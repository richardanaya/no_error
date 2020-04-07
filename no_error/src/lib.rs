#![no_std]
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(fake_call_site)]
pub use no_error_macro::error;

pub trait Error {
    fn description(&self) -> &'static str;
    fn source(&self) -> &'static str;
}

impl Error for (&'static str,&'static str) {
    fn description(&self) -> &'static str {
        self.0
    }

    fn source(&self) -> &'static str {
        self.1
    }
}