// Copyright (c) 2022-2023 Kamyar Mohajerani

use super::*;
use crate::crate_prelude::*;

def_type!(InOutType);

impl InOutType {
    #[inline(always)]
    /// Creates an HW inout type in the context associated with element.
    pub fn new(element: &impl Ty) -> Self {
        Self::try_from_raw(unsafe { hwInOutTypeGet(element.raw()) }).unwrap()
    }

    #[inline(always)]
    /// Returns the bitwidth of an integer type.
    pub fn element_type(&self) -> Type {
        Type::try_from_raw(unsafe { hwInOutTypeGetElementType(self.raw()) }).unwrap()
    }
}

impl TyIsa for InOutType {
    /// If the type is an HW inout.
    fn isa(ty: &Type) -> bool {
        unsafe { hwTypeIsAInOut(ty.raw()) }
    }
}
