// #include "circt/Conversion/ExportVerilog.h"
// #include "circt/Conversion/Passes.h"
// #include "circt/Dialect/Comb/CombDialect.h"
// #include "circt/Dialect/FIRRTL/FIRParser.h"
// #include "circt/Dialect/FIRRTL/FIRRTLDialect.h"
// #include "circt/Dialect/FIRRTL/FIRRTLOps.h"
// #include "circt/Dialect/FIRRTL/Passes.h"
// #include "circt/Dialect/HW/HWDialect.h"
// #include "circt/Dialect/HW/HWOps.h"
// #include "circt/Dialect/SV/SVDialect.h"
// #include "circt/Dialect/SV/SVPasses.h"
// #include "circt/Support/LoweringOptions.h"
// #include "mlir/IR/AsmState.h"
// #include "mlir/IR/BuiltinOps.h"
// #include "mlir/Parser/Parser.h"
// #include "mlir/Pass/Pass.h"
// #include "mlir/Pass/PassManager.h"
// #include "mlir/Support/FileUtilities.h"
// #include "mlir/Support/Timing.h"
// #include "mlir/Support/ToolUtilities.h"
// #include "mlir/Transforms/GreedyPatternRewriteDriver.h"
// #include "mlir/Transforms/Passes.h"
// #include "llvm/Support/FileSystem.h"
// #include "llvm/Support/InitLLVM.h"
// #include "llvm/Support/Path.h"
// #include "llvm/Support/SourceMgr.h"
// #include "llvm/Support/ToolOutputFile.h"
// #include "circt/Dialect/HW/HWAttributes.h"
// #include "circt/Dialect/HW/HWOps.h"
// #include "circt/Dialect/HW/HWTypes.h"
// #include "circt/Support/LLVM.h"
// #include "mlir/CAPI/IR.h"
// #include "mlir/CAPI/Registration.h"
// #include "mlir/CAPI/Support.h"
// #include "mlir/CAPI/Pass.h"
// #include "llvm/Support/SourceMgr.h"
// #include "mlir/Support/LogicalResult.h"
#include "circt/Conversion/ExportVerilog.h"
#include "circt/Conversion/Passes.h"
#include "circt/Dialect/Comb/CombDialect.h"
#include "circt/Dialect/FIRRTL/CHIRRTLDialect.h"
#include "circt/Dialect/FIRRTL/FIRParser.h"
#include "circt/Dialect/FIRRTL/FIRRTLDialect.h"
#include "circt/Dialect/FIRRTL/FIRRTLOps.h"
#include "circt/Dialect/FIRRTL/Passes.h"
#include "circt/Dialect/HW/HWDialect.h"
#include "circt/Dialect/HW/HWOps.h"
#include "circt/Dialect/SV/SVDialect.h"
#include "circt/Dialect/SV/SVPasses.h"
#include "circt/Dialect/Seq/SeqDialect.h"
#include "circt/Dialect/Seq/SeqPasses.h"
#include "circt/Support/LoweringOptions.h"
#include "circt/Support/LoweringOptionsParser.h"
#include "circt/Support/Version.h"
#include "circt/Transforms/Passes.h"
#include "mlir/Bytecode/BytecodeReader.h"
#include "mlir/Bytecode/BytecodeWriter.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"
#include "mlir/IR/AsmState.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/Parser/Parser.h"
#include "mlir/Pass/Pass.h"
#include "mlir/Pass/PassInstrumentation.h"
#include "mlir/Pass/PassManager.h"
#include "mlir/Support/FileUtilities.h"
#include "mlir/Support/Timing.h"
#include "mlir/Support/ToolUtilities.h"
#include "mlir/Transforms/GreedyPatternRewriteDriver.h"
#include "mlir/Transforms/Passes.h"
#include "llvm/Support/Chrono.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/InitLLVM.h"
#include "llvm/Support/Path.h"
#include "llvm/Support/PrettyStackTrace.h"
#include "llvm/Support/SourceMgr.h"
#include "llvm/Support/ToolOutputFile.h"

#include "firrtl.h"


using namespace mlir;
using namespace circt;


/// Create a simple canonicalizer pass.
std::unique_ptr<Pass> createSimpleCanonicalizerPass()
{
    mlir::GreedyRewriteConfig config;
    config.useTopDownTraversal = true;
    config.enableRegionSimplification = false;
    return mlir::createCanonicalizerPass(config);
}

static std::string toString(MlirStringRef cstr)
{
    return std::string(cstr.data, cstr.length);
}

/**
 ******************************
 *  C-API
 ******************************
**/

DEFINE_C_API_PTR_METHODS(MlirDefaultTimingManager, mlir::DefaultTimingManager)
DEFINE_C_API_PTR_METHODS(MlirTimingManager, mlir::TimingManager)
DEFINE_C_API_PTR_METHODS(MlirTimingScope, mlir::TimingScope)

MlirDefaultTimingManager mlirCreateDefaultTimingManager()
{
    return wrap(new DefaultTimingManager());
}

MlirTimingManager mlirDefaultTimingManagerGetAsTimingManager(MlirDefaultTimingManager tm)
{
    return wrap(static_cast<TimingManager *>(unwrap(tm)));
}

MlirTimingScope mlirTimingManagerGetRootScope(MlirTimingManager tm)
{
    return wrap(new TimingScope((unwrap(tm)->getRootTimer())));
}

void PassManagerEnableTimingFromTiminigManagerRootScope(PassManager pm, MlirDefaultTimingManager tm)
{
    auto ts = unwrap(tm)->getRootScope();
    unwrap(pm)->enableTiming(ts);
}

void PassManagerEnableTiming(PassManager pm, MlirTimingScope ts)
{
    unwrap(pm)->enableTiming(*unwrap(ts));
}

#define DEFINE_PASS_NOARGS(name)               \
    MlirPass mlirCreateFIRRTL##name()              \
    {                                          \
        return wrap(create##name##Pass().release()); \
    }

#define DEFINE_PASS_1ARG(name, arg_type)          \
    MlirPass mlirCreateFIRRTL##name(arg_type arg)     \
    {                                             \
        return wrap(create##name##Pass(arg).release()); \
    }

#define DEFINE_PASS_NS_NOARGS(ns, name)            \
    MlirPass ns##Create##name()                    \
    {                                              \
        return wrap(ns::create##name##Pass().release()); \
    }

#define DEFINE_PASS_NS_1ARG(ns, name, arg_type)       \
    MlirPass ns##Create##name(arg_type arg)           \
    {                                                 \
        return wrap(ns::create##name##Pass(arg).release()); \
    }


DEFINE_PASS_NOARGS(SimpleCanonicalizer)


MlirPass circtCreateExportVerilogToFilePass(MlirStringRef outfile_name)
{
    auto outputFilename = toString(outfile_name);
    std::string errorMessage;
    std::unique_ptr<llvm::ToolOutputFile> outputFile = openOutputFile(outputFilename, &errorMessage);
    if (!outputFile)
    {
        llvm::errs() << errorMessage << "\n SWITCHING TO STDOUT!\n";
        return wrap(createExportVerilogPass().release());
    }

    return wrap(createExportVerilogPass(outputFile.release()->os()).release());
}

///////////////////////////

MlirModule firrtlParseFile(MlirContext ctx, MlirStringRef input_filename)
{
    MLIRContext &context = *unwrap(ctx);
    std::string inputFilename = toString(input_filename);

    // Set up the input file.
    std::string errorMessage;
    auto input = openInputFile(inputFilename, &errorMessage);
    if (!input)
    {
        llvm::errs() << "Error in openInputFile" << errorMessage << "\n";
        return MlirModule(); // FIXME ?
    }

    // Register our dialects.
    // context.loadDialect<firrtl::FIRRTLDialect, hw::HWDialect, comb::CombDialect,
    // sv::SVDialect>();

    llvm::SourceMgr sourceMgr;
    sourceMgr.AddNewSourceBuffer(std::move(input), llvm::SMLoc());
    // if (!verifyDiagnostics)
    // {
    //     SourceMgrDiagnosticHandler sourceMgrHandler(sourceMgr, &context);
    //     return processBuffer(context, ts, sourceMgr, outputFile);
    // }

    SourceMgrDiagnosticVerifierHandler sourceMgrHandler(sourceMgr, &context);
    context.printOpOnDiagnostic(true);
    //
    // BEGIN processBuffer
    //

    unsigned numAnnotationFiles = 0;
    // TODO: inputAnnotationFilenames

    OwningModuleRef module;

    firrtl::FIRParserOptions options;
    options.ignoreInfoLocators = false; // TODO
    options.rawAnnotations = true;      // new Anno
    options.numAnnotationFiles = 0;

    auto modRef = importFIRFile(sourceMgr, &context, options);

    return wrap(modRef.release());

    // return sourceMgrHandler.verify().succeeded();
}

bool outputMlir(MlirModule module, MlirStringRef outfile_name)
{
    auto outputFilename = toString(outfile_name);
    std::string errorMessage;
    auto outputFile = openOutputFile(outputFilename, &errorMessage); // FIXME
    if (!outputFile)
    {
        llvm::errs() << errorMessage << "\n";
        return false;
    }
    unwrap(module)->print(outputFile->os());
    return true;
}
