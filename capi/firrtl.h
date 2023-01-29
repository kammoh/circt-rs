#ifndef _EXT_CAPI_FIRRTL__H_
#define _EXT_CAPI_FIRRTL__H_

#include "mlir-c/IR.h"
#include "mlir/CAPI/Registration.h"

#ifdef __cplusplus
extern "C"
{
#endif

    /*
 * *****************************
 *  C-API
 * *****************************
**/

#define DEFINE_C_API_STRUCT(name, storage) \
    struct name                            \
    {                                      \
        storage *ptr;                      \
    };                                     \
    typedef struct name name

    DEFINE_C_API_STRUCT(MlirDefaultTimingManager, void);
    DEFINE_C_API_STRUCT(MlirTimingManager, void);
    DEFINE_C_API_STRUCT(MlirTimingScope, void);

#undef DEFINE_C_API_STRUCT

#define DECLARE_PASS(name) \
    MLIR_CAPI_EXPORTED MlirPass mlirCreateFIRRTL##name();

    // #define DECLARE_PASS_WITHARGS(ns, name, args) \
//     MLIR_CAPI_EXPORTED MlirPass ns##Create##name(args);

    MLIR_CAPI_EXPORTED MlirDefaultTimingManager mlirCreateDefaultTimingManager();

    /// Register a set of useful command-line options that can be used to configure
    /// a `DefaultTimingManager`. The values of these options can be applied via the
    /// `applyDefaultTimingManagerCLOptions` method.
    // MLIR_CAPI_EXPORTED void mlirRegisterDefaultTimingManagerCLOptions();

    // MLIR_CAPI_EXPORTED void mlirApplyDefaultTimingManagerCLOptions(MlirDefaultTimingManager tm);

    MLIR_CAPI_EXPORTED MlirTimingScope mlirTimingManagerGetRootScope(MlirTimingManager tm);

    MLIR_CAPI_EXPORTED MlirTimingManager mlirDefaultTimingManagerGetAsTimingManager(MlirDefaultTimingManager tm);

    MLIR_CAPI_EXPORTED void mlirPassManagerEnableTiming(MlirPassManager pm, MlirTimingScope ts);

    MLIR_CAPI_EXPORTED void mlirPassManagerEnableTimingFromTiminigManagerRootScope(MlirPassManager pm, MlirDefaultTimingManager tm);

    // MLIR_CAPI_EXPORTED void mlirApplyDefaultTimingPassManagerCLOptions(MlirPassManager pm);

    MLIR_CAPI_EXPORTED MlirModule firrtlParseFile(MlirContext ctx, MlirStringRef input_filename);

    // firrtl etc passes
    // DECLARE_PASS(circt, ExportVerilogPass)
    // DECLARE_PASS_WITHARGS(circt, ExportVerilogToFilePass, MlirStringRef outfile_name)
    // DECLARE_PASS(firrtl, BlackBoxMemoryPass)
    // DECLARE_PASS(firrtl, CheckCombCyclesPass)
    // DECLARE_PASS_WITHARGS(firrtl, EmitOMIRPass, MlirStringRef outfile)
    // DECLARE_PASS(firrtl, ExpandWhensPass)
    // DECLARE_PASS(firrtl, IMConstPropPass)
    // DECLARE_PASS(firrtl, InferResetsPass)
    // DECLARE_PASS(firrtl, InferWidthsPass)
    // DECLARE_PASS(firrtl, InlinerPass)
    // DECLARE_PASS(firrtl, LowerCHIRRTLPass)
    // DECLARE_PASS(firrtl, PrefixModulesPass)
    DECLARE_PASS(SimpleCanonicalizer)
    // DECLARE_PASS_WITHARGS(firrtl, LowerFIRRTLToHWPass, bool anno_warn)
    // DECLARE_PASS_WITHARGS(firrtl, LowerFIRRTLTypesPass, bool repl_seq_mem)
    // DECLARE_PASS(sv, HWCleanupPass)
    // DECLARE_PASS(sv, HWLegalizeModulesPass)
    // DECLARE_PASS(sv, PrettifyVerilogPass)
    // DECLARE_PASS_WITHARGS(sv, HWMemSimImplPass, bool repl_seq_mem)

    // PM
    // MLIR_CAPI_EXPORTED MlirOpPassManager pmNestAsFirrtlCircuitOp(MlirPassManager pm);
    // MLIR_CAPI_EXPORTED MlirOpPassManager pmNestAsFirrtlFModuleOp(MlirPassManager pm);
    // MLIR_CAPI_EXPORTED MlirOpPassManager pmNestAsHWModuleOp(MlirPassManager pm);
    // MLIR_CAPI_EXPORTED MlirOpPassManager pmNestAsFirrtlCircuitOpThenAsFModuleOp(MlirPassManager pm);

#ifdef __cplusplus
}
#endif

int firtool(int argc, char **argv);

#endif // _EXT_CAPI_FIRRTL__H_