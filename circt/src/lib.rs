#[macro_use]
pub(crate) mod macros;

pub mod builtin;
pub mod cf;
pub mod comb;
pub mod firrtl;
pub mod fsm;
pub mod func;
pub mod hw;
pub mod mlir;
pub mod seq;
pub mod sv;
pub mod wrap_raw;
use std::{fmt::Write, path::Path};

pub use builtin::*;
use circt_sys::*;
pub use mlir::string::*;
pub use mlir::*;
pub(crate) use wrap_raw::*;

// pub mod prelude {
// }

mod crate_prelude {
    pub use crate::mlir::*;
    pub(crate) use crate::wrap_raw::{HasRaw, ToRawVec, WrapRaw, WrapRawPtr};
    pub use crate::{comb, firrtl, fsm, hw, seq, sv};
}

/// Emits split Verilog files for the specified module into the given directory.
pub fn export_split_verilog<P>(module: &Module, directory: &P) -> LogicalResult
where
    P: AsRef<Path> + ?Sized,
{
    LogicalResult::from_raw(unsafe {
        mlirExportSplitVerilog(
            module.raw(),
            StringRef::from_str(directory.as_ref().to_str().unwrap()).raw(),
        )
    })
}

pub fn export_verilog<W: Write>(module: &Module, w: &mut W) -> LogicalResult {
    LogicalResult::from_raw(unsafe {
        let fmt = FormatterCallback::new(w);
        // Emits verilog for the specified module using the provided callback and user data
        mlirExportVerilog(module.raw(), fmt.callback(), fmt.user_data())
    })
}

#[cfg(test)]
mod tests {
    use crate::hw::HwModuleOp;

    use super::*;

    #[test]
    fn register_hw() {
        let ctx = OwnedContext::default();
        assert_eq!(ctx.num_loaded_dialects(), 1);

        let hw_handle = hw::dialect().unwrap();
        let hw = hw_handle.load_dialect(&ctx).unwrap();
        let hw2 = hw_handle.load_dialect(&ctx).unwrap();
        assert_eq!(ctx.num_loaded_dialects(), 2);
        assert_eq!(hw, hw2);

        let seq_handle = seq::dialect().unwrap();
        let seq = seq_handle.load_dialect(&ctx).unwrap();
        let seq2 = seq_handle.load_dialect(&ctx).unwrap();
        assert_eq!(ctx.num_loaded_dialects(), 3);
        assert_eq!(seq, seq2);

        hw::register_passes();
        seq::register_passes();
    }

    #[test]
    fn hw_types() {
        let ctx = OwnedContext::default();
        assert_eq!(ctx.num_loaded_dialects(), 1);

        let hw_handle = hw::dialect().unwrap();
        let _ = hw_handle.load_dialect(&ctx).unwrap();

        let i8_type = ctx.get_integer_type(8).unwrap();
        let i8_io_type = hw::get_inout_type(i8_type).unwrap();

        let _ = hw::inout_type_get_element(i8_io_type).unwrap();
        assert!(!hw::is_inout_type(i8_type));
        assert!(hw::is_inout_type(i8_io_type));

        let scope = "myscope";
        let name = "myname";

        let type_alias = hw::get_type_alias(scope, name, i8_type).unwrap();
        assert!(hw::is_alias_type(type_alias));
        let canonical_type = hw::alias_type_get_canonical_type(type_alias).unwrap();
        assert_eq!(canonical_type, i8_type);
        let inner_type = hw::alias_type_get_inner_type(type_alias).unwrap();
        assert_eq!(inner_type, i8_type);
        let the_scope = hw::alias_type_get_scope(type_alias).unwrap();
        assert_eq!(the_scope, scope);
        let the_name = hw::alias_type_get_name(type_alias).unwrap();
        assert_eq!(the_name, name);
    }

}
