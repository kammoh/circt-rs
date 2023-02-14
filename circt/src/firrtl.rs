// Copyright (c) 2022-2023 Kamyar Mohajerani

use circt_sys::registerFirrtlPasses;
define_dialect!(firrtl);

pub fn register_firrtl_passes() {
    unsafe { registerFirrtlPasses() }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use miette::IntoDiagnostic;

    #[test]
    fn test_firrtl() -> miette::Result<()> {
        let ctx = OwnedContext::default();
        let firrtl_handle = firrtl::dialect();
        firrtl_handle
            .load(&ctx)
            .expect("failed to load FIRRTL dialect");
        assert!(ctx.num_loaded_dialects() >= 2);
        Ok(())
    }
    #[test]
    fn test_firtool() {
        let ctx = OwnedContext::default();

        // let pm = mlirPassManagerCreate(ctx);

        // let tm = mlirCreateDefaultTimingManager();
        // let ts = mlirTimingManagerGetRootScope(mlirDefaultTimingManagerGetAsTimingManager(tm));
        // mlirPassManagerEnableTiming(pm, ts);

        // let verify_passes = true;
        // mlirPassManagerEnableVerifier(pm, verify_passes);

        // let module = firrtlParseFile(ctx, "test.fir".into());
        // assert!(!module.ptr.is_null());

        // pm.nest("firrtl.circuit")
        //     .nest("firrtl.module")
        //     .add_pass(mlirCreateTransformsCSE());

        // pm.nest("firrtl.circuit")
        //     .nest("firrtl.module")
        //     .add_pass(mlirCreateFIRRTLLowerCHIRRTL());
        // pm.nest("firrtl.circuit")
        //     .add_pass(mlirCreateFIRRTLInferWidths());
        // pm.nest("firrtl.circuit")
        //     .add_pass(mlirCreateFIRRTLInferResets());
        // mlirOpPassManagerAddOwnedPass(
        //     pm.nest("firrtl.circuit"),
        //     mlirCreateFIRRTLPrefixModules(),
        // );
        // mlirOpPassManagerAddOwnedPass(
        //     pm.nest("firrtl.circuit"),
        //     mlirCreateFIRRTLBlackBoxMemory(),
        // );
        // mlirOpPassManagerAddOwnedPass(
        //     pm.nest("firrtl.circuit"),
        //     mlirCreateFIRRTLLowerFIRRTLTypes(),
        // );
        // pm.nest("firrtl.circuit")
        //     .nest("firrtl.module")
        //     .add_pass(mlirCreateFIRRTLExpandWhens());

        // mlirOpPassManagerAddOwnedPass(
        //     pm.nest("firrtl.circuit"),
        //     mlirCreateFIRRTLCheckCombCycles(),
        // );

        // pm.nest("firrtl.circuit")
        //     .nest("firrtl.module")
        //     .add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

        // pm.nest("firrtl.circuit")
        //     .add_pass(mlirCreateFIRRTLInliner());

        // pm.nest("firrtl.circuit")
        //     .add_pass(mlirCreateFIRRTLIMConstProp());

        // // createBlackBoxReaderPass

        // // grandCentral

        // pm.nest("firrtl.circuit")
        //     .nest("firrtl.module")
        //     .add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

        // // createCreateSiFiveMetadataPass

        // let omir_pass = mlirCreateFIRRTLEmitOMIR();
        // // (*(omir_pass.ptr)).outputFilename = "out.mir".into();

        // pm.nest("firrtl.circuit").add_pass(omir_pass);

        // pm.add_pass(mlirCreateLowerFIRRTLToHW());
        // pm.add_pass(mlirCreateSVHWMemSimImpl());

        // //createSVExtractTestCodePass

        // let hw_module_pm = pm.nest("hw.module");

        // hw_module_pm.add_pass(mlirCreateSVHWCleanup());
        // hw_module_pm.add_pass(mlirCreateTransformsCSE());
        // hw_module_pm.add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

        // hw_module_pm.add_pass(mlirCreateSVHWLegalizeModules());
        // hw_module_pm.add_pass(mlirCreateSVPrettifyVerilog());

        // // pm.add_pass( circtCreateExportVerilogToFilePass("out.v".into()));
        // pm.add_pass(mlirCreateTransformsViewOpGraphPass());

        // let mut buffer = File::create("pipeline.txt").expect("unable to create file");
        // let mut _d = [0u8];

        // let (user_data, cb) = buffer.into_callback();
        // mlirPrintPassPipeline(pm.into(), Some(cb), user_data);

        // // first run the PassManager
        // assert!(pm.run(module).is_ok());

        // // then export verilog
        // assert!(module
        //     .export_verilog(File::create("out.v").expect("unable to create file"))
        //     .is_ok());
    }
}
