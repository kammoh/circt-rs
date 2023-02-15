// Copyright (c) 2022-2023 Kamyar Mohajerani

use std::borrow::Borrow;

use super::*;
use crate::crate_prelude::*;

pub trait HwTy: HasRaw<RawType = MlirType> {
    /// Return the hardware bit width of a type.
    /// Does not reflect any encoding, padding, or storage scheme, just the bit (and wire width) of a statically-size type.
    /// Reflects the number of wires needed to transmit a value of this type.
    /// Returns `None` if the type is not known or cannot be statically computed.
    fn bitwidth(&self) -> Option<usize> {
        unsafe { hwGetBitWidth(self.raw()) }.try_into().ok()
    }

    /// Return true if the specified type can be used as an HW value type, that is the set of types
    ///  that can be composed together to represent synthesized, hardware but not marker types like
    ///  InOutType or unknown types from other dialects.
    fn is_value_type(&self) -> bool {
        unsafe { hwTypeIsAValueType(self.raw()) }
    }
}

impl Type {
    /// Return true if the specified type can be used as an HW value type, that is
    ///  the set of types that can be composed together to represent synthesized,
    ///  hardware but not marker types like InOutType or unknown types from other dialects.
    pub fn is_hw_value_type(ty: &impl Ty) -> bool {
        unsafe { hwTypeIsAValueType(ty.raw()) }
    }
}

def_type!(ArrayType);

impl ArrayType {
    /// Creates a fixed-size HW array type in the context associated with element_type
    pub fn new(element_type: &impl Ty, size: usize) -> Self {
        Self::try_from_raw(unsafe { hwArrayTypeGet(element_type.raw(), size) }).unwrap()
    }

    // returns the size of an array type
    pub fn size(&self) -> usize {
        unsafe { hwArrayTypeGetSize(self.raw()) }.try_into().unwrap()
    }

    /// returns the element type of an array type
    pub fn element_type(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { hwArrayTypeGetElementType(self.raw()) })
    }
}

impl TyIsa for ArrayType {
    /// If the type is an HW array
    fn isa(ty: &impl HasRaw<RawType = MlirType>) -> bool {
        unsafe { hwTypeIsAArrayType(ty.raw()) }
    }
}

impl HwTy for ArrayType {}

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
    fn isa(ty: &impl HasRaw<RawType = MlirType>) -> bool {
        unsafe { hwTypeIsAInOut(ty.raw()) }
    }
}
impl HwTy for InOutType {}

def_type!(StructType);

impl StructType {
    /// Creates an HW struct type in the context associated with the elements.
    pub fn new_from_raw(ctx: &Context, elements: &[HWStructFieldInfo]) -> Option<Self> {
        Self::try_from_raw(unsafe {
            hwStructTypeGet(ctx.raw(), elements.len() as _, elements.as_ptr())
        })
    }

    /// Creates an HW struct type in the context associated with the elements.
    pub fn new<'a, T: Ty>(
        ctx: &Context,
        elements: impl IntoIterator<Item = impl Borrow<(&'a str, T)>>,
    ) -> Option<Self> {
        let elements: Vec<_> = elements
            .into_iter()
            .map(|tup| HWStructFieldInfo {
                name: Identifier::new(ctx, tup.borrow().0).raw(),
                type_: tup.borrow().1.raw(),
            })
            .collect();
        Self::try_from_raw(unsafe {
            hwStructTypeGet(ctx.raw(), elements.len() as _, elements.as_ptr())
        })
    }

    pub fn field(&self, field_name: &str) -> Option<Type> {
        let field_name = StringRef::from_str(field_name);
        Type::try_from_raw(unsafe { hwStructTypeGetField(self.raw(), field_name.raw()) })
    }

    pub fn field_at(&self, pos: usize) -> Option<(String, Type)> {
        let HWStructFieldInfo { name, type_ } =
            unsafe { hwStructTypeGetFieldNum(self.raw(), pos as _) };
        Some((Identifier::try_from_raw(name)?.to_string(), Type::try_from_raw(type_)?))
    }

    pub fn num_fields(&self) -> usize {
        unsafe { hwStructTypeGetNumFields(self.raw()) }.try_into().unwrap()
    }

    /// Get an iterator over the fields of a struct type.
    pub fn fields(&self) -> Vec<(String, Type)> {
        (0..self.num_fields()).map(|i| self.field_at(i).unwrap()).collect()
    }
}

impl TyIsa for StructType {
    /// If the type is an HW struct.
    fn isa(ty: &impl HasRaw<RawType = MlirType>) -> bool {
        unsafe { hwTypeIsAStructType(ty.raw()) }
    }
}
impl HwTy for StructType {}

def_type!(IntType; doc = "parameterized-width integer. Parameterized integer types are equivalent to the MLIR standard integer type: it is signless, and may be any width integer. This type represents the case when the width is a parameter in the HW dialect sense.");

impl TyIsa for IntType {
    /// If the type is an HW int.
    fn isa(ty: &impl HasRaw<RawType = MlirType>) -> bool {
        unsafe { hwTypeIsAIntType(ty.raw()) }
    }
}

impl HwTy for IntType {}

def_type!(AliasType);

impl AliasType {
    /// Get HW type alias
    pub fn new(scope: &str, name: &str, inner_type: Type) -> Option<Self> {
        Self::try_from_raw(unsafe {
            hwTypeAliasTypeGet(
                StringRef::from_str(scope).raw(),
                StringRef::from_str(name).raw(),
                inner_type.raw(),
            )
        })
    }

    pub fn canonical_type(&self) -> Type {
        Type::try_from_raw(unsafe { hwTypeAliasTypeGetCanonicalType(self.raw()) }).unwrap()
    }

    pub fn inner_type(&self) -> Type {
        Type::try_from_raw(unsafe { hwTypeAliasTypeGetInnerType(self.raw()) }).unwrap()
    }

    pub fn name(&self) -> String {
        StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetName(self.raw()) })
            .map(|sr| sr.to_string())
            .unwrap()
    }

    pub fn scope(&self) -> String {
        StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetScope(self.raw()) })
            .map(|sr| sr.to_string())
            .unwrap()
    }
}

impl TyIsa for AliasType {
    /// If the type is an HW type alias.
    fn isa(ty: &impl HasRaw<RawType = MlirType>) -> bool {
        unsafe { hwTypeIsATypeAliasType(ty.raw()) }
    }
}
impl HwTy for AliasType {}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hw_types() {
        let ctx = OwnedContext::default();
        assert_eq!(ctx.num_loaded_dialects(), 1);

        let hw_handle = hw::dialect();
        let _ = hw_handle.load(&ctx).unwrap();

        let i8_type = ctx.get_integer_type(8).unwrap();
        let i8_io_type = hw::InOutType::new(&i8_type);

        assert_eq!(i8_io_type.element_type(), i8_type);
        assert!(!hw::InOutType::isa(&i8_type));
        assert!(hw::InOutType::isa(&i8_io_type));

        let scope = "myscope";
        let name = "myname";

        let type_alias = hw::AliasType::new(scope, name, i8_type).unwrap();
        assert!(hw::AliasType::isa(&type_alias));
        assert_eq!(type_alias.canonical_type(), i8_type);
        assert_eq!(type_alias.inner_type(), i8_type);
        assert_eq!(type_alias.scope(), scope);
        assert_eq!(type_alias.name(), name);
    }
}
