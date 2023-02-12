// Copyright (c) 2016-2021 Fabian Schuiki

//! Utilities to deal with the MLIR context.

use crate::crate_prelude::*;
use circt_sys::*;

use super::string::StringRef;

wrap_raw_ptr!(Context);

/// An owned MLIR context.
// pub type OwnedContext = Owned<Context>;

impl Context {
    /// Get an interned identifier.
    pub fn get_identifier(&self, ident: &str) -> MlirIdentifier {
        let string_ref = StringRef::from_str(ident);
        unsafe { mlirIdentifierGet(self.raw(), string_ref.raw()) }
    }

    /// Create a new integer type of a given width.
    pub fn get_integer_type(&self, bitwidth: u32) -> Option<Type> {
        Type::try_from_raw(unsafe { mlirIntegerTypeGet(self.raw(), bitwidth) })
    }

    /// Create a new function type.
    pub fn get_function_type(
        &self,
        inputs: impl Iterator<Item = impl Ty>,
        results: impl Iterator<Item = impl Ty>,
    ) -> Option<Type> {
        let inputs: Vec<MlirType> = inputs.into_iter().map(|x| x.raw()).collect();
        let results: Vec<MlirType> = results.into_iter().map(|x| x.raw()).collect();
        Type::try_from_raw(unsafe {
            mlirFunctionTypeGet(
                self.raw(),
                inputs.len() as _,
                inputs.as_ptr(),
                results.len() as _,
                results.as_ptr(),
            )
        })
    }

    /// Create a new struct type.
    pub fn get_struct_type(
        &self,
        fields: impl IntoIterator<Item = (impl AsRef<str>, Type)>,
    ) -> Option<Type> {
        let raw_fields: Vec<_> = fields
            .into_iter()
            .map(|(ident, ty)| HWStructFieldInfo {
                name: self.get_identifier(ident.as_ref()),
                type_: ty.raw(),
            })
            .collect();
        Type::try_from_raw(unsafe {
            hwStructTypeGet(self.raw(), raw_fields.len() as _, raw_fields.as_ptr())
        })
    }

    /// Change whether this MLIR context allows unregistered dialect ops.
    pub fn set_allow_unregistered_dialects(&self, allow: bool) {
        unsafe { mlirContextSetAllowUnregisteredDialects(self.raw(), allow) }
    }

    /// Change whether this MLIR context allows unregistered dialect ops.
    pub fn are_unregistered_dialects_allowed(&self) -> bool {
        unsafe { mlirContextGetAllowUnregisteredDialects(self.raw()) }
    }
    /// Number of loaded dialects. The built-in dialect is always loaded.
    pub fn num_loaded_dialects(&self) -> usize {
        unsafe { mlirContextGetNumLoadedDialects(self.raw()) as _ }
    }
}

impl_create!(Context);
impl_into_owned!(Context);

impl Default for Owned<Context> {
    fn default() -> Self {
        Self(Context::create().unwrap())
    }
}
