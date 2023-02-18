// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

//! MLIR source locations.

use crate::crate_prelude::*;
use circt_sys::*;

use super::string::StringRef;

wrap_raw_ptr!(Location, Clone, Copy);

impl Location {
    /// Create a new unknown source location.
    pub fn new_unknown(ctx: &Context) -> Self {
        Location::try_from_raw(unsafe { mlirLocationUnknownGet(ctx.raw()) }).unwrap()
    }

    /// Create a new location from a file, line, and column.
    pub fn new(ctx: &Context, filename: &str, line: usize, col: usize) -> Self {
        let filename = StringRef::from_str(filename);
        Location::try_from_raw(unsafe {
            mlirLocationFileLineColGet(ctx.raw(), filename.raw(), line as _, col as _)
        })
        .unwrap()
    }

    /// Gets the context that a location was created with.
    pub fn context(&self) -> Context {
        Context::from_raw(unsafe { mlirLocationGetContext(self.raw()) })
    }

    /// Returns the underlying location attribute of this location.
    pub fn attr(&self) -> LocationAttr {
        LocationAttr::try_from_raw(unsafe { mlirLocationGetAttribute(self.0) }).unwrap()
    }

    /// Creates a name location owned by the given context.
    /// If child_loc is `None`, then the behavior is the same as having unknown child location.
    pub fn new_named(ctx: &Context, name: &str, child_loc: Option<&Self>) -> Self {
        let child_loc = child_loc.map_or(
            MlirLocation {
                ptr: std::ptr::null(),
            },
            |loc| loc.raw(),
        );
        let name = StringRef::from_str(name).raw();
        Self::from_raw(unsafe { mlirLocationNameGet(ctx.raw(), name, child_loc) })
    }
}

impl_mlir_print!(Location);

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirLocationEqual(self.0, other.0) }
    }
}
