// Copyright (c) 2016-2021 Fabian Schuiki

pub mod attr;
pub mod block;
pub mod builder;
pub mod context;
pub mod dialect;
pub mod location;
pub mod module;
pub mod operation;
pub mod pass_manager;
pub mod passes;
pub mod string;
pub mod symbol;
pub mod transforms;
pub mod ty;
pub mod value;

use std::error::Error;

pub use attr::*;
pub use block::*;
pub use builder::*;
pub use context::*;
pub use dialect::*;
pub use location::*;
pub use module::*;
pub use operation::*;
pub use pass_manager::*;
pub use passes::*;
pub use string::*;
pub use symbol::*;
pub use ty::*;
pub use value::*;

use crate::crate_prelude::*;
use circt_sys::*;

pub struct Owned<T: IntoOwned>(T);

pub trait IntoOwned {
    fn destroy(&mut self);
}

impl<T: IntoOwned> Drop for Owned<T> {
    fn drop(&mut self) {
        self.0.destroy()
    }
}

impl<T: IntoOwned> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

wrap_raw!(LogicalResult);

impl LogicalResult {
    pub fn success() -> Self {
        Self(MlirLogicalResult { value: 1 })
    }
    pub fn failure() -> Self {
        Self(MlirLogicalResult { value: 0 })
    }

    pub fn to_option<T>(&self, ok: T) -> Option<T> {
        self.is_success().then_some(ok)
    }

    pub fn to_result<T, E: Error>(&self, ok: T, err: E) -> Result<T, E> {
        self.to_option(ok).ok_or(err)
    }

    pub fn is_success(&self) -> bool {
        self.raw().value != 0
    }
}

impl From<bool> for LogicalResult {
    fn from(value: bool) -> Self {
        Self(MlirLogicalResult { value: value as _ })
    }
}

impl From<LogicalResult> for bool {
    fn from(value: LogicalResult) -> Self {
        value.is_success()
    }
}

impl From<LogicalResult> for Option<()> {
    fn from(value: LogicalResult) -> Self {
        value.to_option(())
    }
}

impl<T: Default> From<LogicalResult> for Result<(), T> {
    fn from(value: LogicalResult) -> Self {
        value.to_option(()).ok_or(Default::default())
    }
}

pub type StringRefCallback = unsafe extern "C" fn(MlirStringRef, *mut std::ffi::c_void);

/// for writing or formatting into Unicode-accepting buffers or streams.
pub struct FormatterCallback<'a, W: std::fmt::Write + Sized> {
    /// The received StringRef *must* be be UTF-8 encoded!
    user_data: &'a mut W,
    callback: Option<StringRefCallback>,
}

impl<'a, W> FormatterCallback<'a, W>
where
    W: std::fmt::Write + Sized,
{
    pub fn new(w: &'a mut W) -> Self {
        unsafe extern "C" fn formatter_callback<T: std::fmt::Write + Sized>(
            msg: MlirStringRef,
            use_data: *mut std::ffi::c_void,
        ) {
            let f: &mut T = &mut *(use_data as *mut T);
            let msg = StringRef::try_from_raw(msg).unwrap();
            f.write_str(msg.as_str()).unwrap();
        }

        Self {
            user_data: w,
            callback: Some(formatter_callback::<W>),
        }
    }
    pub fn callback(&self) -> Option<StringRefCallback> {
        self.callback
    }

    /// user data sent to C callback
    pub fn user_data(&self) -> *mut std::ffi::c_void {
        self.user_data as *const _ as *mut _
    }
}

/// print callback for std::io::Write, i.e., byte-oriented sinks.
pub struct IoWriteFormatterCallback<'a, W: std::io::Write + Sized> {
    user_data: &'a mut W,
    callback: Option<StringRefCallback>,
}

impl<'a, W> IoWriteFormatterCallback<'a, W>
where
    W: std::io::Write + Sized,
{
    pub fn new(w: &'a mut W) -> Self {
        unsafe extern "C" fn formatter_callback<T: std::io::Write + Sized>(
            msg: MlirStringRef,
            use_data: *mut std::ffi::c_void,
        ) {
            let f: &mut T = &mut *(use_data as *mut T);
            let msg = StringRef::try_from_raw(msg).unwrap();
            f.write_all(msg.as_bytes()).unwrap();
            f.flush().unwrap();
        }

        Self {
            user_data: w,
            callback: Some(formatter_callback::<W>),
        }
    }

    /// get the callback
    pub fn callback(&self) -> Option<StringRefCallback> {
        self.callback
    }

    /// user data sent to callback
    pub fn user_data(&self) -> *mut std::ffi::c_void {
        self.user_data as *const _ as *mut _
    }
}
