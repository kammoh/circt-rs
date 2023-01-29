#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    unused
)]

pub mod capi {
    use libc::size_t;
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/circt_build/bindings.rs"
    ));
}

pub use libc::size_t;
use capi::*;
use std::hash::{Hash, Hasher};

impl<T> From<T> for MlirStringRef
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let s = value.as_ref().as_bytes();
        unsafe {
            MlirStringRef {
                data: s.as_ptr() as *const _,
                length: s.len() as size_t,
            }
        }
    }
}

pub unsafe fn mlirStringRefToStr<'a, R>(s: MlirStringRef, f: impl Fn(&'a str) -> R + 'a) -> R {
    f(std::str::from_utf8(std::slice::from_raw_parts(
        s.data as *const _,
        s.length as usize,
    ))
    .expect("utf8 string"))
}

pub unsafe fn mlirIdentifierGetFromStr(cx: MlirContext, s: impl AsRef<str>) -> MlirIdentifier {
    mlirIdentifierGet(cx, s.into())
}

pub unsafe fn mlirIdentifierToStr<'a, R>(i: MlirIdentifier, f: impl Fn(&'a str) -> R + 'a) -> R {
    mlirStringRefToStr(mlirIdentifierStr(i), f)
}

impl Eq for MlirValue {}

impl PartialEq for MlirValue {
    fn eq(&self, other: &Self) -> bool {
        match (self.ptr.is_null(), other.ptr.is_null()) {
            (true, true) => true,
            (false, false) => unsafe { mlirValueEqual(*self, *other) },
            _ => false,
        }
    }
}

impl Hash for MlirValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl Eq for MlirType {}

impl PartialEq for MlirType {
    fn eq(&self, other: &Self) -> bool {
        match (self.ptr.is_null(), other.ptr.is_null()) {
            (true, true) => true,
            (false, false) => unsafe { mlirTypeEqual(*self, *other) },
            _ => false,
        }
    }
}

impl Hash for MlirType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

impl Eq for MlirBlock {}

impl PartialEq for MlirBlock {
    fn eq(&self, other: &Self) -> bool {
        match (self.ptr.is_null(), other.ptr.is_null()) {
            (true, true) => true,
            (false, false) => unsafe { mlirBlockEqual(*self, *other) },
            _ => false,
        }
    }
}

impl Hash for MlirBlock {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use capi::*;
    use std::fs::File;

    use super::*;

    #[test]
    fn register_only_hw() {
        unsafe {
            // let context = MlirContext{};
            // assert!(mlirContextGetNumRegisteredDialects(context) == 1);

            // let hwHandle = mlirGetDialectHandle__hw__();
            // let hw = mlirContextGetOrLoadDialect(context, "hw".into());

            // mlirDialectHandleRegisterDialect(hwHandle, context);
            // assert!(context.num_registered_dialects() == 2);
            // assert!(context.num_loaded_dialects() == 1);

            // let hw = mlirContextGetOrLoadDialect(ctx, mlirDialectHandleGetNamespace(hwHandle));
            // assert!(!hw.is_null());

            // assert!(mlirContextGetNumLoadedDialects(ctx) == 2);

            // let alsoRtl = mlirDialectHandleLoadDialect(hwHandle, ctx);
            // assert!(mlirDialectEqual(hw, alsoRtl));

            // // Seq dialect tests.
            // let seqHandle = mlirGetDialectHandle__seq__();
            // mlirDialectHandleRegisterDialect(seqHandle, ctx);
            // mlirDialectHandleLoadDialect(seqHandle, ctx);

            // let seq = mlirContextGetOrLoadDialect(ctx, mlirDialectHandleGetNamespace(seqHandle));
            // assert!(!seq.is_null());

            // let alsoSeq = mlirDialectHandleLoadDialect(seqHandle, ctx);
            // assert!(mlirDialectEqual(seq, alsoSeq));

            // registerSeqPasses();
        }
    }
    // #[test]
    // fn test_hw_types() {
    //     unsafe {
    //         let ctx = mlirContextCreate();
    //         let hwHandle = mlirGetDialectHandle__hw__();
    //         mlirDialectHandleRegisterDialect(hwHandle, ctx);
    //         mlirDialectHandleLoadDialect(hwHandle, ctx);

    //         let i8type = mlirIntegerTypeGet(ctx, 8);
    //         let io8type = hwInOutTypeGet(i8type);
    //         assert!(!io8type.is_null());

    //         let elementType = hwInOutTypeGetElementType(io8type);
    //         assert!(!elementType.is_null());

    //         assert!(!hwTypeIsAInOut(i8type));

    //         assert!(hwTypeIsAInOut(io8type));

    //         let scope = "myscope";
    //         let name = "myname";
    //         let typeAliasType = hwTypeAliasTypeGet(scope.into(), name.into(), i8type);
    //         assert!(!typeAliasType.is_null());
    //         assert!(hwTypeIsATypeAliasType(typeAliasType));
    //         let canonicalType = hwTypeAliasTypeGetCanonicalType(typeAliasType);
    //         assert!(mlirTypeEqual(canonicalType, i8type));
    //         let innerType = hwTypeAliasTypeGetInnerType(typeAliasType);
    //         assert!(mlirTypeEqual(innerType, i8type));

    //         let theScope = hwTypeAliasTypeGetScope(typeAliasType);
    //         assert!(!theScope.length != scope.len());
    //         let theName = hwTypeAliasTypeGetName(typeAliasType);
    //         assert!(!theName.length != name.len());
    //     }
    // }

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
