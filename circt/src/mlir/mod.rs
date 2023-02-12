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
pub mod string;
pub mod symbol;
pub mod ty;
pub mod value;

pub use attr::*;
pub use block::*;
pub use builder::*;
pub use context::*;
pub use dialect::*;
pub use location::*;
pub use module::*;
pub use operation::*;
pub use pass_manager::*;
pub use string::*;
pub use symbol::*;
pub use ty::*;
pub use value::*;

use circt_sys::*;

use crate::crate_prelude::WrapRawPtr;

// Some MLIR objects are created using `_Create`, the ownership is given to the caller,
//  and the caller is responsible to free them using their `_Destroy` function.
// Mlir_ C-binding functions need a copy of the Mlir_ object, which contains a pointer to actual C++ object.
// Is there a way to keep reference count and destroy on drop?!
//
// Types with Create & Destroy (11)
// MlirBlock, MlirContext, MlirDialectRegistry, MlirExecutionEngine, MlirModule, MlirOperation,
//  MlirOpPrintingFlags, MlirPassManager, MlirRegion, MlirSymbolTable, MlirTypeIDAllocator
//

pub struct Owned<T: IntoOwned>(T);

pub trait IntoOwned {
    fn destroy(&mut self);
}

impl<T: IntoOwned> Drop for Owned<T> {
    fn drop(&mut self) {
        println!("Dropping ...");
        // self.0.destroy()
    }
}

impl<T: IntoOwned> std::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
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
            let f: &mut T = std::mem::transmute(use_data);
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
            let f: &mut T = std::mem::transmute(use_data);
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
