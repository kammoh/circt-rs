// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

//! Facilities to deal with MLIR attributes.

use crate::crate_prelude::*;
use circt_sys::*;
use num::Num;
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    ops::Not,
};

wrap_raw_ptr!(Attribute);

impl Attribute {}

wrap_raw!(NamedAttribute);

impl NamedAttribute {
    pub fn new(name: &Identifier, attr: &Attribute) -> Self {
        Self::from_raw(unsafe { mlirNamedAttributeGet(name.raw(), attr.raw()) })
    }

    pub fn try_from_raw(raw: <Self as HasRaw>::RawType) -> Option<Self> {
        raw.attribute.ptr.is_null().not().then_some(Self::from_raw(raw))
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.print(f)
    }
}

impl Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl AttrIsa for Attribute {
    fn isa(_: &impl Attr) -> bool {
        true
    }
}

pub trait Attr: WrapRawPtr<RawType = MlirAttribute> {
    /// Returns an empty attribute.
    fn null() -> Self {
        Self::from_raw(unsafe { mlirAttributeGetNull() })
    }

    fn parse(ctx: &Context, attr: &str) -> Option<Self> {
        Self::try_from_raw(unsafe {
            mlirAttributeParseGet(ctx.raw(), StringRef::from_str(attr).raw())
        })
    }

    /// Get the attribute's MLIR context.
    fn context(&self) -> Context {
        Context::try_from_raw(unsafe { mlirAttributeGetContext(self.raw()) }).unwrap()
    }

    /// Associate a name with this attribute.
    fn to_named(&self, name: &str) -> NamedAttribute {
        NamedAttribute::from_raw(unsafe {
            mlirNamedAttributeGet(self.context().get_identifier(name), self.raw())
        })
    }

    impl_mlir_print_fn!(Attribute);
}

impl<T> Attr for T where T: WrapRawPtr<RawType = MlirAttribute> {}

pub trait AttrIsa: Attr {
    fn isa(_: &impl Attr) -> bool {
        panic!("not implemented!")
    }
}

def_attr!(ArrayAttr [Array]);

impl ArrayAttr {
    /// Creates an array element containing the given list of elements in the given context.
    pub fn new<T: Attr>(ctx: &Context, elements: impl IntoIterator<Item = impl Borrow<T>>) -> Self {
        let elements: Vec<_> = elements.to_raw_vec();
        Self::try_from_raw(unsafe {
            mlirArrayAttrGet(ctx.raw(), elements.len() as _, elements.as_ptr())
        })
        .expect("ArrayAttr::new received null")
    }
}

def_attr!(DictionaryAttr [Dictionary]);

impl DictionaryAttr {
    pub fn new(
        ctx: &Context,
        attrs: impl IntoIterator<Item = impl Borrow<NamedAttribute>>,
    ) -> Self {
        let attrs = attrs.to_raw_vec();
        Self::from_raw(unsafe {
            mlirDictionaryAttrGet(ctx.raw(), attrs.len() as _, attrs.as_slice().as_ptr())
        })
    }

    pub fn num_elements(&self) -> usize {
        unsafe { mlirDictionaryAttrGetNumElements(self.raw()) as _ }
    }

    /// Returns pos-th element of the given dictionary attribute.
    pub fn element(&self, pos: usize) -> NamedAttribute {
        NamedAttribute::from_raw(unsafe { mlirDictionaryAttrGetElement(self.raw(), pos as _) })
    }
}

def_attr!(IntegerAttr [Integer], Clone);

impl IntegerAttr {
    pub fn new(ty: &impl Ty, value: impl Into<i64>) -> Self {
        Self::from_raw(unsafe { mlirIntegerAttrGet(ty.raw(), value.into()) })
    }

    /// Creates an integer attribute of the given type by parsing the given string into an integer value.
    pub fn from_str(ty: &impl Ty, value: &str) -> Self {
        Self::try_from_raw(unsafe {
            mlirIntegerAttrGetFromString(ty.raw(), StringRef::from_str(value).raw())
        })
        .unwrap()
    }

    /// Creates an integer attribute of the given type by parsing the given string into an integer value.
    pub fn from_bigint(ty: &impl Ty, value: impl Num + ToString) -> Self {
        Self::from_str(ty, value.to_string().as_str())
    }
}

def_attr!(OpaqueAttr [Opaque]);

def_attr!(StringAttr [String], Clone);

impl StringAttr {
    /// Creates a string attribute in the given context containing the given string.
    pub fn new(ctx: &Context, string: &str) -> Self {
        Self::try_from_raw(unsafe {
            mlirStringAttrGet(ctx.raw(), StringRef::from_str(string).raw())
        })
        .unwrap()
    }

    /// Creates a string attribute in the given context containing the given string.
    /// Additionally, the attribute has the given type.
    pub fn new_typed(ty: Type, string: &str) -> Self {
        Self::from_raw(unsafe {
            mlirStringAttrTypedGet(ty.raw(), StringRef::from_str(string).raw())
        })
    }

    pub fn get_value(&self) -> String {
        String::from(
            StringRef::try_from_raw(unsafe { mlirStringAttrGetValue(self.raw()) })
                .unwrap()
                .as_str(),
        )
    }
}

def_attr!(SymbolRefAttr [SymbolRef]);

impl SymbolRefAttr {
    /// Creates a flat symbol reference attribute in the given context referencing a symbol identified by the given string.
    pub fn new(ctx: &Context, symbol: &str) -> Self {
        let symbol = StringRef::from_str(symbol);
        Self::try_from_raw(unsafe { mlirFlatSymbolRefAttrGet(ctx.raw(), symbol.raw()) }).unwrap()
    }
}

def_attr!(TypeAttr [Type]);

impl TypeAttr {
    /// Creates a type attribute wrapping the given type in the same context as the type.
    pub fn new(value: &impl HasRaw<RawType = MlirType>) -> Self {
        Self::try_from_raw(unsafe { mlirTypeAttrGet(value.raw()) }).unwrap()
    }

    /// Returns the type stored in the given type attribute.
    pub fn ty(&self) -> Type {
        Type::try_from_raw(unsafe { mlirTypeAttrGetValue(self.raw()) }).unwrap()
    }
}

def_attr!(UnitAttr [Unit]);

impl UnitAttr {
    /// Creates a unit attribute in the given context.
    pub fn new(ctx: &Context) -> Self {
        Self::from_raw(unsafe { mlirUnitAttrGet(ctx.raw()) })
    }
}

def_attr!(LocationAttr[Location]);
