// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

//! An MLIR type.

use crate::crate_prelude::*;
use circt_sys::*;
use std::borrow::Borrow;
use std::fmt::{Debug, Display};

wrap_raw_ptr!(Type, Clone, Copy; doc="An MLIR type");

impl Type {
    pub fn parse(ctx: &Context, type_name: &str) -> Option<Self> {
        Self::try_from_raw(unsafe {
            mlirTypeParseGet(ctx.raw(), StringRef::from_str(type_name).raw())
        })
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.format(f)
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.equal_to(other)
    }
}

impl TyIsa for Type {
    fn isa(_: &Type) -> bool {
        true
    }
}

// generic trait for all MLIR types
pub trait Ty: WrapRawPtr<RawType = MlirType> {
    /// Get the type's MLIR context.
    fn context(&self) -> Context {
        Context::try_from_raw(unsafe { mlirTypeGetContext(self.raw()) }).unwrap()
    }

    fn format(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let fmt = FormatterCallback::new(f);
        unsafe { mlirTypePrint(self.raw(), fmt.callback(), fmt.user_data()) };
        Ok(())
    }

    fn equal_to(&self, other: &Self) -> bool {
        unsafe { mlirTypeEqual(self.raw(), other.raw()) }
    }

    fn as_type(&self) -> Type {
        Type::from_raw(self.raw())
    }
}

/// including for Type (unspecified type)
impl<T> Ty for T where T: WrapRawPtr<RawType = MlirType> {}

pub trait TyIsa: Ty {
    fn isa(_: &Type) -> bool {
        panic!("not implemented")
    }
}

def_type!(IndexType);

impl IndexType {
    /// Creates an index type in the given context. The type is owned by the context.
    pub fn new(ctx: &Context) -> Self {
        Self::try_from_raw(unsafe { mlirIndexTypeGet(ctx.raw()) }).unwrap()
    }
}

impl TyIsa for IndexType {
    /// Checks whether the given type is an index type.
    fn isa(ty: &Type) -> bool {
        unsafe { mlirTypeIsAIndex(ty.raw()) }
    }
}

def_type!(IntegerType);

impl IntegerType {
    pub fn new(ctx: &Context, width: u32) -> Self {
        Self::try_from_raw(unsafe { mlirIntegerTypeGet(ctx.raw(), width as _) }).unwrap()
    }

    /// Returns the bitwidth of an integer type.
    pub fn width(&self) -> u32 {
        unsafe { mlirIntegerTypeGetWidth(self.raw()) as _ }
    }
}

impl TyIsa for IntegerType {
    /// Checks whether the given type is an integer type.
    fn isa(ty: &Type) -> bool {
        unsafe { mlirTypeIsAInteger(ty.raw()) }
    }
}

def_type!(FunctionType);

impl FunctionType {
    pub fn new(
        ctx: &Context,
        inputs: impl Iterator<Item = impl Borrow<Type>>,
        results: impl Iterator<Item = impl Borrow<Type>>,
    ) -> Self {
        let inputs: Vec<_> = inputs.map(|e| e.borrow().raw()).collect();
        let results: Vec<_> = results.map(|e| e.borrow().raw()).collect();
        Self::new_from_raw(ctx, &inputs, &results).unwrap()
    }

    pub fn new_from_raw(ctx: &Context, inputs: &[MlirType], results: &[MlirType]) -> Option<Self> {
        Self::try_from_raw(unsafe {
            mlirFunctionTypeGet(
                ctx.raw(),
                inputs.len() as _,
                inputs.as_ptr(),
                results.len() as _,
                results.as_ptr(),
            )
        })
    }

    /// Returns the number of input types.
    pub fn num_inputs(&self) -> usize {
        unsafe { mlirFunctionTypeGetNumInputs(self.raw()) as _ }
    }
    /// Returns the number of result types.
    pub fn num_results(&self) -> usize {
        unsafe { mlirFunctionTypeGetNumResults(self.raw()) as _ }
    }
    /// Returns the pos-th input type.
    pub fn input(&self, pos: usize) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirFunctionTypeGetInput(self.raw(), pos as _) })
    }

    ///Returns the pos-th result type.
    pub fn result(&self, pos: usize) -> Option<Self> {
        Self::try_from_raw(unsafe { mlirFunctionTypeGetResult(self.raw(), pos as _) })
    }
    /// Return the inputs of a function type.
    pub fn inputs(&self) -> impl Iterator<Item = Self> + '_ {
        (0..self.num_inputs()).map(move |i| self.input(i).unwrap())
    }
    /// Return the results of a function type.
    pub fn results(&self) -> impl Iterator<Item = Self> + '_ {
        (0..self.num_results()).map(move |i| self.result(i).unwrap())
    }
}

impl TyIsa for FunctionType {
    /// Checks whether the given type is a function type.
    fn isa(ty: &Type) -> bool {
        unsafe { mlirTypeIsAFunction(ty.raw()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_type() {
        let ctx = OwnedContext::new();
        let i8 = IntegerType::new(&ctx, 8);
        let my_i1 = IntegerType::new(&ctx, 1);
        assert_eq!(i8.to_string(), "i8");
        assert_eq!(my_i1.to_string(), "i1");
        println!("{}", i8);
        println!("{:?}", my_i1);
    }
}
