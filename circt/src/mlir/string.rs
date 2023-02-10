// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;
use circt_sys::*;
use std::{fmt::Display, marker::PhantomData};

pub struct StringRef<'a>(MlirStringRef, PhantomData<&'a MlirStringRef>)
where
    MlirStringRef: 'a;

impl_has_raw!(StringRef<'a>);

impl<'a> WrapRawPtr for StringRef<'a>
where
    MlirStringRef: 'a,
{
    fn try_from_raw(raw: Self::RawType) -> Option<Self> {
        (!raw.data.is_null()).then_some(StringRef(raw, PhantomData))
    }
}

impl<'a> StringRef<'a>
where
    MlirStringRef: 'a,
{
    pub fn from_str(s: &'a str) -> Self {
        StringRef(
            MlirStringRef {
                data: s.as_ptr() as _,
                length: s.len(),
            },
            PhantomData,
        )
    }

    pub fn as_bytes(&'a self) -> &'a [u8] {
        unsafe { std::slice::from_raw_parts(self.0.data as _, self.0.length) }
    }

    pub fn as_str(&'a self) -> &'a str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}

impl<'a> PartialEq for StringRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirStringRefEqual(self.0, other.0) }
    }
}

wrap_raw_ptr!(Identifier);

impl Identifier {
    /// Gets an identifier with the given string value.
    pub fn new(ctx: &Context, value: &StringRef) -> Self {
        Identifier::from_raw(unsafe { mlirIdentifierGet(ctx.raw(), value.raw()) })
    }
    /// Gets the string value of the identifier.
    pub fn to_string_ref(&self) -> StringRef {
        StringRef::from_raw(unsafe { mlirIdentifierStr(self.raw()) })
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string_ref().as_str())
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        unsafe { mlirIdentifierEqual(self.raw(), other.raw()) }
    }
}

impl Eq for Identifier {}
