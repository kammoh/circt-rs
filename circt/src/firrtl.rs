// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;
use circt_sys::*;

define_dialect!(firrtl);

pub fn register_passes() {
    unsafe { registerFIRRTLPasses() }
}

pub fn register_lower_chirrtl_pass() {
    unsafe { firrtlRegisterLowerCHIRRTLPass() }
}

pub fn create_lower_chirrtl_pass() -> Pass {
    Pass::try_from_raw(unsafe { firrtlCreateLowerCHIRRTLPass() }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlir::cse;
    use miette::IntoDiagnostic;

    #[test]
    fn test_firrtl() -> miette::Result<()> {
        let ctx = OwnedContext::default();
        let firrtl_handle = firrtl::dialect();
        firrtl_handle.load(&ctx).expect("failed to load FIRRTL dialect");
        assert!(ctx.num_loaded_dialects() >= 2);
        Ok(())
    }
    #[test]
    fn test_firtool() -> miette::Result<()> {
        let ctx = OwnedContext::default();
        hw::register_passes();
        hw::register_arith_passes();
        seq::register_passes();
        sv::register_passes();
        firrtl::dialect().load(&ctx).unwrap();
        firrtl::register_passes();
        firrtl::register_lower_chirrtl_pass();

        let builder = OpBuilder::new(&ctx);
        let module = Module::create(builder.loc());

        let pm = OwnedPassManager::new(&ctx);
        pm.enable_verifier(true);

        pm.nest("firrtl.circuit").nest("firrtl.module").add_pass(&cse());

        pm.nest("firrtl.circuit")
            .nest("firrtl.module")
            .add_pass(&create_lower_chirrtl_pass());
        pm.nest("firrtl.circuit") //
            .parse_pass("firrtl-infer-widths")
            .into_diagnostic()?;
        pm.nest("firrtl.circuit") //
            .parse_pass("firrtl-infer-resets")
            .into_diagnostic()?
            .parse_pass("firrtl-prefix-modules")
            .into_diagnostic()?;

        pm.run(&module).unwrap();

        Ok(())
    }

    // #[test]
    // fn torture_test_firrtl() {
    //     for _ in 0..1000 {
    //         test_firtool().unwrap();
    //     }
    // }
}
