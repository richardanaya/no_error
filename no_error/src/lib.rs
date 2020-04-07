#![no_std]
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack(fake_call_site)]
pub use no_error_macro::error_message;

pub type ErrorCode = u32;
pub type ErrorMessage = (&'static str, &'static str);

pub enum NoError {
    Code(ErrorCode),
    Message((&'static str, &'static str)),
}

pub type Result<T> = core::result::Result<T, NoError>;

pub trait Error {
    fn code(&self) -> Option<ErrorCode>;
    fn description(&self) -> &'static str;
    fn source(&self) -> &'static str;
}

impl Error for NoError {
    #[inline]
    fn code(&self) -> Option<ErrorCode> {
        match self {
            NoError::Code(c) => Some(*c),
            NoError::Message(_) => None,
        }
    }

    #[inline]
    fn description(&self) -> &'static str {
        match self {
            NoError::Code(_) => "ERR_CODE\0",
            NoError::Message(m) => m.0,
        }
    }

    #[inline]
    fn source(&self) -> &'static str {
        match self {
            NoError::Code(_) => "\0",
            NoError::Message(m) => m.1,
        }
    }
}

#[macro_export]
macro_rules! error_code {
    ( $x:expr ) => {{
        Err(no_error::NoError::Code($x))
    }};
}
