//! This file provides a public interface to a Clang library for extracting
//! high-level symbol information from source files without exposing the full
//! Clang C++ API.

use super::cxstring_into_string;
use clang_sys::*;
use std::fmt;
use std::os::raw::c_int;

pub struct Cursor {
    // TODO
}

/// An `Index` is an environment for a set of translation units that will
/// typically end up linked together in one final binary.
pub struct Index {
    x: CXIndex,
}

impl Index {
    /// Construct a new `Index`.
    ///
    /// The `pch` parameter controls whether declaration in pre-compiled headers
    /// are included when enumerating a translation unit's "locals".
    ///
    /// A "local" declaration is one that belongs in the translation unit itself
    /// and not in a precompiled header that was used by the translation unit.
    ///
    /// the `diag` parameter controls whether debugging diagnostics are enabled.
    pub fn new(pch: bool, diag: bool) -> Index {
        unsafe {
            Index {
                x: clang_createIndex(pch as c_int, diag as c_int),
            }
        }
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Index {{ }}")
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe {
            clang_disposeIndex(self.x);
        }
    }
}

/// The type of an element in the abstract syntax tree.
#[derive(Clone, Copy)]
pub struct Type {
    x: CXType,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        unsafe { clang_equalTypes(self.x, other.x) != 0 }
    }
}

impl Eq for Type {}

impl fmt::Debug for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Type {
    /// Gets this type's kind
    pub fn kind(&self) -> CXTypeKind {
        self.x.kind
    }

    pub fn declaration(&self) -> Cursor {
        unimplemented!()
    }

    /// Pretty-print the underlying type using the rules of the language of the
    /// translation unit from which it came.
    ///
    /// If the type is invalid, an empty string is returned.
    pub fn spelling(&self) -> String {
        let s = unsafe { cxstring_into_string(clang_getTypeSpelling(self.x)) };

        // TODO
        // Clang 5.0 introduced changes in the spelling API so it returned the
        // full qualified name. Let's undo that here.
        // if s.split("::").all(|s|
        s
    }

    /// Determine whether a `Type` has the "const" qualifier set, without
    /// looking through typedefs that may have added "const" at a different
    /// level.
    pub fn is_const(&self) -> bool {
        unsafe { clang_isConstQualifiedType(self.x) != 0 }
    }

    /// Determine whether a `Type` has the "volatile" qualifier set, without
    /// looking through typedefs that may have added "volatile" at a different
    /// level.
    pub fn is_volatile(&self) -> bool {
        unsafe { clang_isVolatileQualifiedType(self.x) != 0 }
    }

    /// Determine whether a `Type` has the "restrict" qualifier set, without
    /// looking through typedefs that may have added "restrict" at a different
    /// level.
    pub fn is_restrict(&self) -> bool {
        unsafe { clang_isRestrictQualifiedType(self.x) != 0 }
    }

    /// Is this type a variadic function type?
    pub fn is_variadic(&self) -> bool {
        unsafe { clang_isFunctionTypeVariadic(self.x) != 0 }
    }

    /// Is this type a POD (plain old data) type?
    pub fn is_pod(&self) -> bool {
        unsafe { clang_isPODType(self.x) != 0 }
    }

    #[inline]
    fn is_non_deductible_auto_type(&self) -> bool {
        debug_assert_eq!(self.kind(), CXType_Auto);
        unimplemented!()
    }
}
