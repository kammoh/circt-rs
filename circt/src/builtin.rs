// Copyright (c) 2016-2021 Fabian Schuiki

//! mlir Operations

use std::borrow::Borrow;

use crate::crate_prelude::*;

def_operation!(UnrealizedConversionCastOp, "builtin.unrealized_conversion_cast");

impl UnrealizedConversionCastOp {
    /// Create a new unrealized conversion cast operation.
    pub fn new(
        builder: &mut OpBuilder,
        values: impl IntoIterator<Item = impl Borrow<Value>>,
        result_tys: impl IntoIterator<Item = impl Borrow<Type>>,
    ) -> Option<Self> {
        builder.build_with(|_, state| {
            for value in values {
                state.add_operand(value.borrow());
            }
            for rty in result_tys {
                state.add_result(rty.borrow());
            }
        })
    }
}
