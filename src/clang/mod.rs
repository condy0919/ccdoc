//! A higher level Clang API built on top of the generated binding in the
//! `clang_sys` modules

use clang_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_longlong, c_uint, c_ulong, c_ulonglong};

// pub mod build_system;
// pub use build_system::*;
pub mod index;

/// Error Codes returned by libclang routines.
///
/// `Success` is the only error code indicating success. Other error codes,
/// including not yet assigned non-zero values, indicate errors.
#[repr(usize)]
pub enum ErrorCode {
    /// No error.
    Success = 0,
    /// A generic error code, no further details are available.
    ///
    /// Errors of this kind can get their own specific error code in future
    /// libclang versions.
    Failure = 1,
    /// libclang crashed while performing the requested operation.
    Crashed = 2,
    /// The function detected that the arguments violate the function contract.
    InvalidArguments = 3,
    /// An AST deserialization error has occurred.
    ASTReadError = 4,
}

/// A cursor into Clang AST, pointing to an AST node.
///
/// We call the AST node pointed to by the cursor the cursor's "referent"
#[derive(Copy, Clone)]
pub struct Cursor {
    x: CXCursor,
}

impl Cursor {}

fn cxstring_to_string_leaky(s: CXString) -> String {
    if s.data.is_null() {
        return String::from("");
    }

    let c_str = unsafe { CStr::from_ptr(clang_getCString(s) as *const _) };
    c_str.to_string_lossy().into_owned()
}

fn cxstring_into_string(s: CXString) -> String {
    let ret = cxstring_to_string_leaky(s);
    unsafe { clang_disposeString(s) };
    ret
}
