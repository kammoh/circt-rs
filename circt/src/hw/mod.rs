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
use crate::mlir::string::StringRef;
use circt_sys::*;
use std::convert::TryInto;

define_dialect!(hw);

pub fn register_arith_passes() {
    unsafe { registerHWArithPasses() }
}

pub fn register_passes() {
    unsafe { registerHWPasses() }
}

pub mod passes {
    use crate::*;

    /// Flattens hw::Structure typed in- and output ports.
    pub fn flatten_io() -> Pass {
        Pass::try_from_raw(unsafe { hwCreateFlattenIOPass() }).unwrap()
    }

    /// Specializes instances of parametric hw.modules
    pub fn hw_specialize() -> Pass {
        Pass::try_from_raw(unsafe { hwCreateHWSpecializePass() }).unwrap()
    }
}

/// Return true if the specified type can be used as an HW value type, that is the set of types
///  that can be composed together to represent synthesized, hardware but not marker types like
///  InOutType or unknown types from other dialects.
pub fn is_value_type(ty: Type) -> bool {
    unsafe { hwTypeIsAValueType(ty.raw()) }
}
