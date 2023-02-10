// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;
use circt_sys::*;

def_attr!(InnerRefAttr);

/// Encapsulates references to inner symbols.
/// This attribute stores the parent symbol and the inner symbol, providing a uniform type for storing and manipulating references to inner symbols.
/// see https://circt.llvm.org/docs/RationaleSymbols/
impl InnerRefAttr {
    pub fn new(module_name: &StringAttr, inner_sym: &StringAttr) -> Option<Self> {
        Self::try_from_raw(unsafe { hwInnerRefAttrGet(module_name.raw(), inner_sym.raw()) })
    }

    pub fn name(&self) -> StringAttr {
        StringAttr::try_from_raw(unsafe { hwInnerRefAttrGetName(self.raw()) }).unwrap()
    }
    pub fn module(&self) -> StringAttr {
        StringAttr::try_from_raw(unsafe { hwInnerRefAttrGetModule(self.raw()) }).unwrap()
    }
}

impl AttrIsa for InnerRefAttr {
    /// Checks whether the given attribute is an integer attribute.
    fn isa(attr: &Attribute) -> bool {
        unsafe { hwAttrIsAInnerRefAttr(attr.raw()) }
    }
}

def_attr!(ParamDeclAttr);

impl ParamDeclAttr {
    pub fn new(name: &str, ty: impl Ty, value: impl Attr) -> Self {
        Self::try_from_raw(unsafe {
            hwParamDeclAttrGet(StringRef::from_str(name).raw(), ty.raw(), value.raw())
        })
        .unwrap()
    }

    pub fn name(&self) -> Option<StringRef> {
        StringRef::try_from_raw(unsafe { hwParamDeclAttrGetName(self.0) })
    }
    pub fn ty(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { hwParamDeclAttrGetType(self.0) })
    }

    pub fn isa(attr: &impl Attr) -> bool {
        unsafe { hwAttrIsAParamDeclAttr(attr.raw()) }
    }
}

def_attr!(ParamDeclRefAttr);

impl ParamDeclRefAttr {
    pub fn new(ctx: &Context, name: &str) -> Self {
        Self::try_from_raw(unsafe {
            hwParamDeclRefAttrGet(ctx.raw(), StringRef::from_str(name).raw())
        })
        .unwrap()
    }

    pub fn name(&self) -> Option<StringRef> {
        StringRef::try_from_raw(unsafe { hwParamDeclRefAttrGetName(self.0) })
    }

    pub fn ty(&self) -> Option<Type> {
        Type::try_from_raw(unsafe { hwParamDeclRefAttrGetType(self.0) })
    }

    pub fn isa(attr: &impl Attr) -> bool {
        unsafe { hwAttrIsAParamDeclRefAttr(attr.raw()) }
    }
}

def_attr!(ParamVerbatimAttr);

impl ParamVerbatimAttr {
    pub fn new(text: StringAttr) -> Self {
        Self::try_from_raw(unsafe { hwParamVerbatimAttrGet(text.raw()) }).unwrap()
    }

    pub fn isa(attr: &impl Attr) -> bool {
        unsafe { hwAttrIsAParamVerbatimAttr(attr.raw()) }
    }
}
