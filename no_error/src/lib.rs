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
    fn cstr_description(&self) -> *const u8;
    fn cstr_source(&self) -> *const u8;
}

const GENERIC_ERROR_CODE: &'static str = "ERR_CODE\0";
const EMPTY: &'static str = "\0";

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
            NoError::Code(_) => &GENERIC_ERROR_CODE[..GENERIC_ERROR_CODE.len() - 1],
            NoError::Message(m) => &m.0[..m.0.len() - 1],
        }
    }

    #[inline]
    fn cstr_description(&self) -> *const u8 {
        match self {
            NoError::Code(_) => GENERIC_ERROR_CODE.as_ptr(),
            NoError::Message(m) => m.0.as_ptr(),
        }
    }

    #[inline]
    fn source(&self) -> &'static str {
        match self {
            NoError::Code(_) => &EMPTY[..EMPTY.len() - 1],
            NoError::Message(m) => &m.1[..m.1.len() - 1],
        }
    }

    #[inline]
    fn cstr_source(&self) -> *const u8 {
        match self {
            NoError::Code(_) => EMPTY.as_ptr(),
            NoError::Message(m) => m.1.as_ptr(),
        }
    }
}

#[macro_export]
macro_rules! error_code {
    ( $x:expr ) => {{
        Err(no_error::NoError::Code($x))
    }};
}
