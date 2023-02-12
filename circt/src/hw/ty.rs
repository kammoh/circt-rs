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
        unsafe { hwArrayTypeGetSize(self.raw()) }
            .try_into()
            .unwrap()
    }

    /// returns the element type of an array type
    pub fn element_type(&self) -> Option<Type> {
        Type::try_from_raw(
            unsafe { hwArrayTypeGetElementType(self.raw()) }
        )
    }
}

impl TyIsa for ArrayType {
    /// If the type is an HW array
    fn isa(ty: &Type) -> bool {
        unsafe { hwTypeIsAArrayType(ty.raw()) }
    }
}

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
        Some((
            Identifier::try_from_raw(name)?.to_string(),
            Type::try_from_raw(type_)?,
        ))
    }

    pub fn num_fields(&self) -> usize {
        unsafe { hwStructTypeGetNumFields(self.raw()) }
            .try_into()
            .unwrap()
    }
}

impl TyIsa for StructType {
    /// If the type is an HW struct.
    fn isa(ty: &Type) -> bool {
        unsafe { hwTypeIsAStructType(ty.raw()) }
    }
}

def_type!(IntType; doc = "parameterized-width integer. Parameterized integer types are equivalent to the MLIR standard integer type: it is signless, and may be any width integer. This type represents the case when the width is a parameter in the HW dialect sense.");

impl TyIsa for IntType {
    /// If the type is an HW int.
    fn isa(ty: &Type) -> bool {
        unsafe { hwTypeIsAIntType(ty.raw()) }
    }
}

def_type!(AliasType);

impl AliasType {
    pub fn canonical_type(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { hwTypeAliasTypeGetCanonicalType(self.raw()) })
    }

    pub fn inner_type(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { hwTypeAliasTypeGetInnerType(self.raw()) })
    }

    pub fn name(&self) -> Option<String> {
        StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetName(self.raw()) })
            .map(|sr| sr.to_string())
    }

    pub fn scope(&self) -> Option<String> {
        StringRef::try_from_raw(unsafe { hwTypeAliasTypeGetScope(self.raw()) })
            .map(|sr| sr.to_string())
    }
}

impl TyIsa for AliasType {
    /// If the type is an HW type alias.
    fn isa(ty: &Type) -> bool {
        unsafe { hwTypeIsATypeAliasType(ty.raw()) }
    }
}
