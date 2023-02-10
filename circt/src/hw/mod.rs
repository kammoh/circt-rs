// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

mod attr;
mod module;
mod ops;
mod ty;
pub use attr::*;
pub use module::*;
pub use ops::*;
pub use ty::*;

use crate::crate_prelude::*;
use crate::mlir::string::{Identifier, StringRef};
use circt_sys::*;
use std::convert::TryInto;

define_dialect!(hw);

pub fn register_hw_arith_passes() {
    unsafe { registerHWArithPasses() }
}

/// Creates an HW inout type in the context associated with element.
pub fn get_inout_type(ty: Type) -> Option<Type> {
    Type::try_from_raw(unsafe { hwInOutTypeGet(ty.raw()) })
}

/// If the type is an HW inout.
pub fn is_inout_type(ty: Type) -> bool {
    unsafe { hwTypeIsAInOut(ty.raw()) }
}

/// Returns the element type of an inout type.
pub fn inout_type_get_element(ty: Type) -> Option<Type> {
    if !is_inout_type(ty) {
        return None;
    }
    Type::try_from_raw(unsafe { hwInOutTypeGetElementType(ty.raw()) })
}

/// Get the bit width of an HW type.
pub fn bit_width(ty: Type) -> Option<usize> {
    unsafe { hwGetBitWidth(ty.raw()) }.try_into().ok()
}

/// Check if a type is an HW array type.
pub fn is_array_type(ty: Type) -> bool {
    unsafe { hwTypeIsAArrayType(ty.raw()) }
}

/// Return true if the specified type can be used as an HW value type, that is the set of types
///  that can be composed together to represent synthesized, hardware but not marker types like
///  InOutType or unknown types from other dialects.
pub fn is_value_type(ty: Type) -> bool {
    unsafe { hwTypeIsAValueType(ty.raw()) }
}

/// Get HW type alias
pub fn get_type_alias(scope: &str, name: &str, inner_type: Type) -> Option<Type> {
    let scope = StringRef::from_str(scope);
    let name = StringRef::from_str(name);
    Type::try_from_raw(unsafe { hwTypeAliasTypeGet(scope.raw(), name.raw(), inner_type.raw()) })
}

/// If the type is an HW type alias.
pub fn is_alias_type(ty: Type) -> bool {
    unsafe { hwTypeIsATypeAliasType(ty.raw()) }
}

pub fn alias_type_get_canonical_type(ty: Type) -> Option<Type> {
    if !is_alias_type(ty) {
        return None;
    }
    Type::try_from_raw(unsafe { hwTypeAliasTypeGetCanonicalType(ty.raw()) })
}

pub fn alias_type_get_inner_type(ty: Type) -> Option<Type> {
    if !is_alias_type(ty) {
        return None;
    }
    Type::try_from_raw(unsafe { hwTypeAliasTypeGetInnerType(ty.raw()) })
}

pub fn alias_type_get_scope(ty: Type) -> Option<String> {
    if !is_alias_type(ty) {
        return None;
    }
    let string_ref = StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetScope(ty.raw()) });
    string_ref.map(|s| s.as_str().to_string())
}

pub fn alias_type_get_name(ty: Type) -> Option<String> {
    if !is_alias_type(ty) {
        return None;
    }
    let string_ref = StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetName(ty.raw()) });
    string_ref.map(|s| s.as_str().to_string())
}

/// Create a new array type.
pub fn get_array_type(element: &Type, size: usize) -> Option<Type> {
    Type::try_from_raw(unsafe { hwArrayTypeGet(element.raw(), size as _) })
}

/// Get the element type of an array type.
pub fn array_type_element(ty: Type) -> Option<Type> {
    Type::try_from_raw(unsafe { hwArrayTypeGetElementType(ty.raw()) })
}

/// Get the size of an array type.
pub fn array_type_size(ty: Type) -> usize {
    unsafe { hwArrayTypeGetSize(ty.raw()) as _ }
}

/// Check if a type is an HW struct type.
pub fn is_struct_type(ty: Type) -> bool {
    unsafe { hwTypeIsAStructType(ty.raw()) }
}

/// If the type is an HW int.
pub fn is_int_type(ty: Type) -> bool {
    unsafe { hwTypeIsAIntType(ty.raw()) }
}

/// Get the number of fields in a struct type.
pub fn struct_type_size(ty: Type) -> usize {
    unsafe { hwStructTypeGetNumFields(ty.raw()) as _ }
}

/// Get a field of a struct type.
pub fn struct_type_field(ty: Type, offset: usize) -> (String, Type) {
    let info = unsafe { hwStructTypeGetFieldNum(ty.raw(), offset as _) };
    let ident = Identifier::try_from_raw(info.name).unwrap();
    (ident.to_string(), Type::try_from_raw(info.type_).unwrap())
}

/// Get an iterator over the fields of a struct type.
pub fn struct_type_fields(ty: Type) -> Vec<(String, Type)> {
    (0..struct_type_size(ty))
        .map(|i| struct_type_field(ty, i).clone())
        .collect()
}
