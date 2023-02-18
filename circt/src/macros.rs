// Copyright (c) 2022-2023 Kamyar Mohajerani

macro_rules! define_dialect {
    ($name:ident) => {
        paste::paste!{
            pub fn dialect() -> crate::DialectHandle {
                use crate::WrapRawPtr;
                super::mlir::DialectHandle::try_from_raw(unsafe { circt_sys::[< mlirGetDialectHandle__ $name __>]() }).unwrap()
            }
            pub fn get_namespace() {}
        }
    };
}

macro_rules! impl_has_raw {
    ($name:ident $(<$lt:lifetime>)? , $inner:ident) => {
        impl$(<$lt>)? crate::HasRaw for $name$(<$lt>)? $(where circt_sys::$inner: $lt)? {
            type RawType = circt_sys::$inner;
            #[inline(always)]
            fn raw(&self) -> Self::RawType {
                self.0
            }
            #[inline(always)]
            fn raw_ref(&self) -> &Self::RawType {
                &self.0
            }
            #[inline(always)]
            fn raw_mut(&mut self) -> &mut Self::RawType {
                &mut self.0
            }
        }

        impl$(<$lt>)? crate::HasRaw for &$name$(<$lt>)? $(where circt_sys::$inner: $lt)? {
            type RawType = circt_sys::$inner;
            #[inline(always)]
            fn raw(&self) -> Self::RawType {
                self.0
            }
            #[inline(always)]
            fn raw_ref(&self) -> &Self::RawType {
                &self.0
            }
            #[inline(always)]
            fn raw_mut(&mut self) -> &mut Self::RawType {
                panic!("not mutable!")
            }
        }

        impl$(<$lt>)? crate::HasRaw for &mut $name$(<$lt>)? $(where circt_sys::$inner: $lt)? {
            type RawType = circt_sys::$inner;
            #[inline(always)]
            fn raw(&self) -> Self::RawType {
                self.0
            }
            #[inline(always)]
            fn raw_ref(&self) -> &Self::RawType {
                &self.0
            }
            #[inline(always)]
            fn raw_mut(&mut self) -> &mut Self::RawType {
                &mut self.0
            }
        }

        impl$(<$lt>)? AsRef<circt_sys::$inner> for $name$(<$lt>)? {
            fn as_ref(&self) -> &circt_sys::$inner {
                use crate::wrap_raw::HasRaw;
                self.raw_ref()
            }
        }
    };
    ($name:ident $(<$lt:lifetime>)?) => {
        paste::paste! {
            impl_has_raw!($name $(<$lt>)?, [<Mlir $name>]);
        }
    };
}

macro_rules! wrap_raw {
    ($name:ident => $inner:ident $(, $derives:ident)* $(; doc=$doc:tt)?) => {
        #[derive(Clone $(, $derives)*)]
        $(#[doc = $doc ])?
        pub struct $name(circt_sys::$inner);

        impl_has_raw!($name, $inner);

        impl crate::WrapRaw for $name {
            #[inline(always)]
            fn from_raw(raw: Self::RawType) -> Self {
                Self(raw)
            }
        }
    };
    ($name:ident $(, $derives:ident)* $(; doc=$doc:tt)?) => {
        paste::paste! {
            wrap_raw!($name => [<Mlir $name>] $(, $derives)* $(; doc=$doc)?);
        }
    };
}

macro_rules! wrap_raw_ptr {
    ($name:ident => $inner:ident / $ptr:ident $(+ $other:ident)* $(,$derives:ident)* $(; doc=$doc:tt)?) => {
        #[derive($($derives,)*)]
        $(#[doc = $doc ])?
        pub struct $name(circt_sys::$inner $(, $other)*);

        impl_has_raw!($name, $inner);

        impl crate::WrapRawPtr for $name {
            #[inline(always)]
            fn try_from_raw(raw: Self::RawType) -> Option<Self> {
                (!raw.$ptr.is_null()).then_some(Self(raw $(, $other::default())*))
            }
        }
    };
    ($name:ident => $inner:ident $(+ $other:ident)* $(,$args:ident)* $(; doc=$doc:tt)?) => {
        wrap_raw_ptr!($name => $inner / ptr $(+ $other)* $(,$args)* $(; doc=$doc)?);
    };
    ($name:ident $(+ $other:ident)* $(, $args:ident)* $(; doc=$doc:tt)?) => {
        paste::paste! {
            wrap_raw_ptr!($name => [<Mlir $name>] / ptr $(+ $other)* $(, $args)* $(; doc=$doc)?);
        }
    }
}

macro_rules! impl_into_owned {
    ($name:ident) => {
        paste::paste! {
            pub type [<Owned $name >] = Owned<$name>;
            #[doc = "Takes a `" $name "` owned by the caller and destroys it."]
            impl IntoOwned for $name {
                fn destroy(&mut self) {
                    unsafe { [<mlir $name Destroy>](self.raw()) }
                }
            }
        }
    };
}

macro_rules! impl_create {
    ($name:ident) => {
        paste::paste! {
            impl Default for $name {
                fn default() -> Self {
                    Self::try_from_raw(unsafe { [<mlir $name Create>]() }).unwrap()
                }
            }
        }
    };
}

/// Define a type.
macro_rules! def_type {
    ($name:ident $(; doc=$doc:tt)?) => {
        paste::paste! {
            wrap_raw_ptr!($name => MlirType, Clone, Copy $(; doc=$doc)?);
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    self.print(f)
                }
            }

            impl std::fmt::Debug for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self)
                }
            }

            impl From<$name> for Type {
                fn from(value: $name) -> Self {
                    Self::from_raw(value.raw())
                }
            }

            // impl From<&$name> for Type {
            //     fn from(value: &$name) -> Self {
            //         Self::from_raw(value.raw())
            //     }
            // }

            impl TryFrom<Type> for $name {
                type Error = simple_error::SimpleError;

                fn try_from(value: Type) -> Result<Self, Self::Error> {
                    paste::paste!{
                        Self::isa(&value)
                        .then_some(Self::from_raw(value.raw()))
                        .ok_or(Self::Error::new(format!("Type {} is not a `{}`", value, stringify!($name))))
                    }
                }
            }

            impl TryFrom<&Type> for $name {
                type Error = simple_error::SimpleError;

                fn try_from(value: &Type) -> Result<Self, Self::Error> {
                    paste::paste!{
                        Self::isa(&value)
                        .then_some(Self::from_raw(value.raw()))
                        .ok_or(Self::Error::new(format!("Type {} is not a `{}`", value, stringify!($name))))
                    }
                }
            }
        }
    };
}

macro_rules! impl_mlir_print_fn {
    ($name:ident) => {
        paste::paste!{
            fn print(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let formatter = FormatterCallback::new(f);
                unsafe { [<mlir $name Print>](self.raw(), formatter.callback(), formatter.user_data()) };
                Ok(())
            }
        }
    };
}
macro_rules! impl_display_debug {
    ($name:ident) => {

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.print(f)
            }
        }
        
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }
    };
}

macro_rules! impl_mlir_print {
    ($name:ident) => {
        impl $name{
            impl_mlir_print_fn!($name);
        }
        impl_display_debug!($name);
    };
}

macro_rules! def_attr {
    (hw :: $name:ident $(, $derives:ident)* ) => {
        def_attr!(hw :: $name [$name:Attr]  $(, $derives)* );
    };
    ($ns:ident :: $name:ident $(, $derives:ident)* ) => {
        def_attr!($ns :: $name [$name:Attribute]  $(, $derives)* );
    };
    ($ns:ident :: $name:ident [$isa_name:ident] $(, $derives:ident)* ) => {
        def_attr!($ns :: $name [$isa_name:Attribute]  $(, $derives)* );
    };
    ($ns:ident :: $name:ident [$isa_name:ident : $isa_kind:ident] $(, $derives:ident)* ) => {
        wrap_raw_ptr!($name => MlirAttribute $(, $derives)*);

        impl From<$name> for Attribute {
            fn from(value: $name) -> Self {
                Attribute::from_raw(value.raw())
            }
        }

        impl TryFrom<Attribute> for $name {
            type Error = simple_error::SimpleError;

            fn try_from(value: Attribute) -> Result<Self, Self::Error> {
                paste::paste!{
                    Self::isa(&value)
                    .then_some(Self::from_raw(value.raw()))
                    .ok_or(Self::Error::new(format!("Attribute: {} is not a `{}`", value, stringify!($name))))
                }
            }
        }

        impl AttrIsa for $name {
            impl_isa!($ns :: $isa_name => $isa_kind);
        }
    };
    ($name:ident $([$isa_name:ident $(:$isa_kind:ident)?])? $(, $derives:ident)* ) => {
        def_attr!(mlir:: $name $([$isa_name $(:$isa_kind)? ])? $(, $derives)* );
    };
}

macro_rules! impl_isa {
    ($ns:ident :: $name:ident => $kind:ident) => {
        // #[doc = "Checks whether the given `" $kind "` is a `" $name "`"]
        fn isa(attr: &impl Attr) -> bool {
            paste::paste!{
                unsafe { [< $ns:lower $kind IsA $name>](attr.raw()) }
            }
        }
    };
    ($name:ident => $kind:ident) => {
        impl_isa!(mlir::$name => $kind );
    };
}

macro_rules! def_val {
    ($name:ident $(, $derives:ident)* ) => {
        wrap_raw_ptr!($name => MlirValue $(, $derives)*);

        impl From<$name> for Value {
            fn from(value: $name) -> Self {
                Value::from_raw(value.raw())
            }
        }

        impl TryFrom<Value> for $name {
            type Error = simple_error::SimpleError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                paste::paste!{
                    Self::isa(&value)
                    .then_some(Self::from_raw(value.raw()))
                    .ok_or(SimpleError::new(format!("Value is not a `{}`", stringify!($name))))
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.print(f)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.equal_to(other)
            }
        }

    };
}

// Copyright (c) 2016-2021 Fabian Schuiki

/// Define an operation.
macro_rules! def_operation {
    ($name:ident, $operation_name:expr $(; doc=$doc:tt)?) => {
        wrap_raw_ptr!($name => MlirOperation, Clone, Copy $(; doc=$doc)? );

        impl mlir::NamedOp for $name {
            const OP_NAME: Option<&'static str> = Some($operation_name);
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use crate::mlir::operation::Op;
                self.print(f, false);
                Ok(())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use crate::mlir::operation::Op;
                self.print(f, true);
                Ok(())
            }
        }

        impl From<$name> for mlir::Operation
        {
            fn from(value: $name) -> Self {
                use crate::wrap_raw::{HasRaw, WrapRawPtr};
                mlir::Operation::from_raw(value.raw())
            }
        }
    };
}

/// Define an operation with a single result.
macro_rules! def_operation_single_result {
    ($name:ident, $operation_name:expr) => {
        def_operation!($name, $operation_name);
        impl_op_single_result!($name);
    };
}

macro_rules! impl_op_single_result {
    ($name:ident) => {
        impl $name {
            pub fn result(&self) -> mlir::Value {
                use crate::mlir::operation::Op;
                self.result_at(0).unwrap()
            }
        }
    };
}

macro_rules! impl_op_build_many_to_one {
    ($name:ident) => {
        impl $name {
            pub fn build(
                builder: &mut mlir::OpBuilder,
                args: impl IntoIterator<Item = impl std::borrow::Borrow<Value>>,
            ) -> Option<Self> {
                builder.build_with(|_, result| {
                    let mut first_arg = true;
                    for arg in args.into_iter() {
                        let arg = arg.borrow();
                        result.add_operand(arg);
                        if first_arg {
                            result.add_result(&arg.ty());
                            first_arg = false;
                        }
                    }
                })
            }
        }
    };
}

macro_rules! def_operation_many_to_one {
    ($name:ident, $operation_name:expr) => {
        def_operation!($name, $operation_name);
        impl_op_build_many_to_one!($name);
        impl_op_single_result!($name);
    };
}

/// Define a unary operation with the result type inherited from the operand.
#[allow(unused_macros)]
macro_rules! def_simple_unary_operation {
    ($name:ident, $operation_name:expr) => {
        def_operation!($name, $operation_name);
        impl_op_single_result!($name);

        impl $name {
            pub fn build(builder: &mut crate::OpBuilder, arg: &crate::Value) -> Option<Self> {
                builder.build_with(|_, result| {
                    use crate::mlir::value::Val;
                    result.add_operand(arg);
                    result.add_result(&arg.ty());
                })
            }
        }
    };
}

/// Define a binary operation with the result type inherited from the first
/// operand.
macro_rules! def_simple_binary_operation {
    ($name:ident, $operation_name:expr) => {
        def_operation!($name, $operation_name);
        impl_op_single_result!($name);

        impl $name {
            pub fn build(builder: &mut OpBuilder, lhs: &Value, rhs: &Value) -> Option<Self> {
                builder.build_with(|_, result| {
                    result.add_operand(lhs);
                    result.add_operand(rhs);
                    result.add_result(&lhs.ty());
                })
            }
        }
    };
}

/// Define a binary operation with an explicit result type.
macro_rules! def_binary_operation_explicit_result {
    ($name:ident, $operation_name:expr) => {
        def_operation!($name, $operation_name);

        impl $name {
            pub fn build(
                builder: &mut OpBuilder,
                ty: &impl Ty,
                lhs: &impl Val,
                rhs: &impl Val,
            ) -> Option<Self> {
                builder.build_with(|_, result| {
                    result.add_operand(lhs);
                    result.add_operand(rhs);
                    result.add_result(ty);
                })
            }
        }
    };
}
