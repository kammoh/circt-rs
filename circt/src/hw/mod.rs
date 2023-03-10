// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

//! The hw dialect provides unifying structure and abstractions that are useful for a wide range of hardware modeling problems.
//! It allows other dialects to “mix in” with it to provide higher level functionality.
//! hw is roughly akin to the “std” dialect in MLIR (but better curated).
//! See https://circt.llvm.org/docs/Dialects/HW/RationaleHW/ and https://circt.llvm.org/docs/Dialects/HW/ for more details.

mod attr;
mod ops;
mod structure;
mod ty;
pub use attr::*;
pub use ops::*;
pub use structure::*;
pub use ty::*;

use crate::crate_prelude::*;
use crate::mlir::string::StringRef;
use circt_sys::*;
use std::convert::TryInto;

define_dialect!(hw);

pub fn register_arith_passes() {
    unsafe { registerHWArithPasses() }
}

pub fn register_hw_passes() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_hw() {
        let ctx = OwnedContext::default();
        assert_eq!(ctx.num_loaded_dialects(), 1);

        let hw_handle = hw::dialect();
        let hw = hw_handle.load(&ctx).unwrap();
        let hw2 = hw_handle.load(&ctx).unwrap();
        assert_eq!(ctx.num_loaded_dialects(), 2);
        assert_eq!(hw, hw2);

        let seq_handle = seq::dialect();
        let seq = seq_handle.load(&ctx).unwrap();
        let seq2 = seq_handle.load(&ctx).unwrap();
        assert_eq!(ctx.num_loaded_dialects(), 3);
        assert_eq!(seq, seq2);

        hw::register_hw_passes();
        seq::register_passes();
    }
}
