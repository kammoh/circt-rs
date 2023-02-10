// Copyright (c) 2016-2021 Fabian Schuiki

//! An MLIR value.

use crate::crate_prelude::*;
use circt_sys::*;
use simple_error::SimpleError;
use std::{
    fmt::{Debug, Display},
    ops::Not,
};

wrap_raw_ptr!(Value, Clone, Copy; doc="An MLIR Value");

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.equal_to(other)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.print(f)
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Val for Value {
    fn isa(value: &Value) -> bool {
        !value.raw().ptr.is_null().not()
    }
}

pub trait Val: HasRaw<RawType = MlirValue> {
    /// Return the type of this value.
    fn ty(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { mlirValueGetType(self.raw()) })
    }

    fn first_use(&self) -> Option<OpOperand> {
        OpOperand::try_from_raw(unsafe { mlirValueGetFirstUse(self.raw()) })
    }

    fn print(&self, w: &mut impl std::fmt::Write) -> std::fmt::Result {
        let formatter = FormatterCallback::new(w);

        // Prints a value by sending chunks of the string representation and forwarding userData to callback`.
        // Note that the callback may be called several times with consecutive chunks of the string.
        unsafe { mlirValuePrint(self.raw(), formatter.callback(), formatter.user_data()) };
        Ok(())
    }

    fn equal_to(&self, other: &impl Val) -> bool {
        match (self.raw().ptr.is_null(), other.raw().ptr.is_null()) {
            (true, true) => true,
            (false, false) => unsafe { mlirValueEqual(self.raw(), other.raw()) },
            _ => false,
        }
    }

    fn isa(_: &Value) -> bool {
        panic!("`isa()` is not implemented for this Value type!")
    }

    /// Prints the value to the standard error stream.
    fn dump(&self) {
        unsafe { mlirValueDump(self.raw()) }
    }
}

def_val!(BlockArgument, Clone, Copy);

impl Val for BlockArgument {
    /// Returns true if the value is a block argument, false otherwise.
    fn isa(value: &Value) -> bool {
        unsafe { mlirValueIsABlockArgument(value.raw()) }
    }
}

def_val!(OpResult, Clone, Copy);

impl OpResult {
    /// Returns an operation that produced this value as its result.
    pub fn owner(&self) -> Option<Operation> {
        Operation::try_from_raw(unsafe { mlirOpResultGetOwner(self.raw()) })
    }
    /// Returns the position of the value in the list of results of the operation that produced it.
    pub fn result_position(&self) -> usize {
        unsafe { mlirOpResultGetResultNumber(self.raw()) }
            .try_into()
            .unwrap()
    }
}

impl Val for OpResult {
    /// Returns true if the value is an operation result, false otherwise.
    fn isa(value: &Value) -> bool {
        unsafe { mlirValueIsAOpResult(value.raw()) }
    }
}
