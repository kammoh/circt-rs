// Copyright (c) 2022-2023 Kamyar Mohajerani

//! The sv dialect provides direct access to a wide variety of SystemVerilog constructs, including behavioral constructs, syntactic sugar constructs, and even idioms like ifdef blocks.

use std::{path::Path, fmt::Write};

use crate::crate_prelude::*;
use circt_sys::*;

define_dialect!(sv);

pub fn register_passes() {
    unsafe { registerSVPasses() }
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
