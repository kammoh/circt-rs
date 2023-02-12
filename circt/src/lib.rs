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

        seq::register_seq_passes();
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

    #[test]
    fn test_pm() {
        let ctx = OwnedContext::default();
        let pm = PassManager::new(&ctx);
        hw::dialect().unwrap().load_dialect(&ctx).unwrap();
        hw::register_hw_arith_passes();

        let loc = Location::new_unknown(&ctx);

        let module = Module::create(&loc);
        // let module = ModuleOp::new(&ctx);
        // let module_op = module.operation();

        let mut op_state =
            OperationState::new(module.operation().name().to_string_ref().as_str(), &loc);

        let op = HwModuleOp::create(&mut op_state);

        // op_state.

        for pipeline in &["builtin.module(lower-hwarith-to-hw)"] {
            pm.parse(pipeline).expect("parse failed");

            // let r = pm.run(&module);
            // assert!(r.is_success());
        }
        let out_dir = Path::new("test_module");
        std::fs::create_dir_all(out_dir).unwrap();
        export_split_verilog(&module, &out_dir);
    }

    // #[test]
    // fn test_firrtl() {
    //     unsafe {
    //         let ctx = mlirContextCreate();
    //         let firrtlHandle = mlirGetDialectHandle__firrtl__();

    //         let firrtl =
    //             mlirContextGetOrLoadDialect(ctx, mlirDialectHandleGetNamespace(firrtlHandle));

    //         assert!(firrtl.is_null());

    //         mlirDialectHandleRegisterDialect(firrtlHandle, ctx);

    //         assert!(mlirContextGetNumRegisteredDialects(ctx) == 2);
    //         assert!(mlirContextGetNumLoadedDialects(ctx) == 1);
    //         mlirContextDestroy(ctx);
    //     }
    // }
    // #[test]
    // fn test_firtool() {
    //     let context = Context::default();

    //     unsafe {
    //         let ctx = context.ctx;

    //         mlirGetDialectHandle__std__()
    //             .register_and_load(&context)
    //             .unwrap();
    //         mlirGetDialectHandle__hw__()
    //             .register_and_load(&context)
    //             .unwrap();
    //         mlirGetDialectHandle__sv__()
    //             .register_and_load(&context)
    //             .unwrap();
    //         mlirGetDialectHandle__firrtl__()
    //             .register_and_load(&context)
    //             .unwrap();

    //         let pm = mlirPassManagerCreate(ctx);

    //         let tm = mlirCreateDefaultTimingManager();
    //         let ts = mlirTimingManagerGetRootScope(mlirDefaultTimingManagerGetAsTimingManager(tm));
    //         mlirPassManagerEnableTiming(pm, ts);

    //         let verify_passes = true;
    //         mlirPassManagerEnableVerifier(pm, verify_passes);

    //         let module = firrtlParseFile(ctx, "test.fir".into());
    //         assert!(!module.ptr.is_null());

    //         pm.nest("firrtl.circuit")
    //             .nest("firrtl.module")
    //             .add_pass(mlirCreateTransformsCSE());

    //         pm.nest("firrtl.circuit")
    //             .nest("firrtl.module")
    //             .add_pass(mlirCreateFIRRTLLowerCHIRRTL());
    //         pm.nest("firrtl.circuit")
    //             .add_pass(mlirCreateFIRRTLInferWidths());
    //         pm.nest("firrtl.circuit")
    //             .add_pass(mlirCreateFIRRTLInferResets());
    //         mlirOpPassManagerAddOwnedPass(
    //             pm.nest("firrtl.circuit"),
    //             mlirCreateFIRRTLPrefixModules(),
    //         );
    //         mlirOpPassManagerAddOwnedPass(
    //             pm.nest("firrtl.circuit"),
    //             mlirCreateFIRRTLBlackBoxMemory(),
    //         );
    //         mlirOpPassManagerAddOwnedPass(
    //             pm.nest("firrtl.circuit"),
    //             mlirCreateFIRRTLLowerFIRRTLTypes(),
    //         );
    //         pm.nest("firrtl.circuit")
    //             .nest("firrtl.module")
    //             .add_pass(mlirCreateFIRRTLExpandWhens());

    //         mlirOpPassManagerAddOwnedPass(
    //             pm.nest("firrtl.circuit"),
    //             mlirCreateFIRRTLCheckCombCycles(),
    //         );

    //         pm.nest("firrtl.circuit")
    //             .nest("firrtl.module")
    //             .add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

    //         pm.nest("firrtl.circuit")
    //             .add_pass(mlirCreateFIRRTLInliner());

    //         pm.nest("firrtl.circuit")
    //             .add_pass(mlirCreateFIRRTLIMConstProp());

    //         // createBlackBoxReaderPass

    //         // grandCentral

    //         pm.nest("firrtl.circuit")
    //             .nest("firrtl.module")
    //             .add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

    //         // createCreateSiFiveMetadataPass

    //         let omir_pass = mlirCreateFIRRTLEmitOMIR();
    //         // (*(omir_pass.ptr)).outputFilename = "out.mir".into();

    //         pm.nest("firrtl.circuit").add_pass(omir_pass);

    //         pm.add_pass(mlirCreateLowerFIRRTLToHW());
    //         pm.add_pass(mlirCreateSVHWMemSimImpl());

    //         //createSVExtractTestCodePass

    //         let hw_module_pm = pm.nest("hw.module");

    //         hw_module_pm.add_pass(mlirCreateSVHWCleanup());
    //         hw_module_pm.add_pass(mlirCreateTransformsCSE());
    //         hw_module_pm.add_pass(mlirCreateFIRRTLSimpleCanonicalizer());

    //         hw_module_pm.add_pass(mlirCreateSVHWLegalizeModules());
    //         hw_module_pm.add_pass(mlirCreateSVPrettifyVerilog());

    //         // pm.add_pass( circtCreateExportVerilogToFilePass("out.v".into()));
    //         pm.add_pass(mlirCreateTransformsViewOpGraphPass());

    //         let mut buffer = File::create("pipeline.txt").expect("unable to create file");
    //         let mut _d = [0u8];

    //         let (user_data, cb) = buffer.into_callback();
    //         mlirPrintPassPipeline(pm.into(), Some(cb), user_data);

    //         // first run the PassManager
    //         assert!(pm.run(module).is_ok());

    //         // then export verilog
    //         assert!(module
    //             .export_verilog(File::create("out.v").expect("unable to create file"))
    //             .is_ok());
    //     }
    // }
    // #[test]
    // fn test_mlir() {
    //     let context = Context::default();
    //     unsafe {
    //         mlirGetDialectHandle__std__().register_and_load(&context);
    //         mlirGetDialectHandle__hw__().register_and_load(&context);
    //         mlirGetDialectHandle__std__().register_and_load(&context);
    //         mlirGetDialectHandle__hw__().register_and_load(&context);

    //         let ctx = context.ctx;
    //         // Create a root MLIR `ModuleOp`.
    //         let module = {
    //             let mut module =
    //                 mlirOperationStateGet("builtin.module".into(), mlirLocationUnknownGet(ctx));
    //             let region = mlirRegionCreate();
    //             mlirRegionAppendOwnedBlock(region, mlirBlockCreate(0, std::ptr::null()));
    //             mlirOperationStateAddOwnedRegions(&mut module, 1, [region].as_ptr());
    //             mlirOperationCreate(&mut module)
    //         };

    //         // // Create a dummy "hw.module" op.
    //         let mut hw_module =
    //             MlirOperationState::get("hw.module", context.new_unknown_location());
    //         let sym_name_ident = mlirIdentifierGet(ctx, "sym_name".into());
    //         let sym_name_attr =
    //             mlirNamedAttributeGet(sym_name_ident, mlirStringAttrGet(ctx, "Foo".into()));
    //         let type_ident = mlirIdentifierGet(ctx, "type".into());
    //         let type_attr = mlirNamedAttributeGet(
    //             type_ident,
    //             mlirTypeAttrGet(mlirFunctionTypeGet(
    //                 ctx,
    //                 0,
    //                 std::ptr::null(),
    //                 0,
    //                 std::ptr::null(),
    //             )),
    //         );
    //         mlirOperationStateAddAttributes(&mut hw_module, 2, [sym_name_attr, type_attr].as_ptr());

    //         let block = mlirBlockCreate(0, std::ptr::null());
    //         let region = mlirRegionCreate();
    //         mlirRegionAppendOwnedBlock(region, block);
    //         mlirOperationStateAddOwnedRegions(&mut hw_module, 1, [region].as_ptr());

    //         let hw_module = hw_module.new_operation().unwrap();
    //         // mlirBlockInsertOwnedOperationBefore(
    //         //     mlirRegionGetFirstBlock(mlirOperationGetRegion(module, 0)),
    //         //     MlirOperation {
    //         //         ptr: std::ptr::null_mut(),
    //         //     },
    //         //     hw_module,
    //         // );

    //         module.dump();
    //     }
    // }
}
