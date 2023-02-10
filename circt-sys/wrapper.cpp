#include "wrapper.h"
#include "mlir/CAPI/IR.h"
#include "mlir/CAPI/Support.h"
#include "mlir/IR/BuiltinTypes.h"

// #include "circt/Dialect/FIRRTL/CHIRRTLDialect.h"
// #include "circt/Dialect/FIRRTL/FIRParser.h"
// #include "circt/Dialect/FIRRTL/FIRRTLDialect.h"
// #include "circt/Dialect/FIRRTL/FIRRTLOps.h"
// #include "circt/Dialect/FIRRTL/Passes.h"
#include "circt/Conversion/ExportVerilog.h"
#include "circt/Conversion/Passes.h"
// #include "circt/Dialect/Comb/CombDialect.h"
// #include "circt/Dialect/FIRRTL/CHIRRTLDialect.h"
// #include "circt/Dialect/FIRRTL/FIRParser.h"
// #include "circt/Dialect/FIRRTL/FIRRTLDialect.h"
// #include "circt/Dialect/FIRRTL/FIRRTLOps.h"
#include "circt/Dialect/FIRRTL/Passes.h"
// #include "circt/Dialect/HW/HWDialect.h"
// #include "circt/Dialect/HW/HWOps.h"
#include "circt/Dialect/SV/SVDialect.h"
#include "circt/Dialect/SV/SVPasses.h"
// #include "circt/Dialect/Seq/SeqDialect.h"
// #include "circt/Dialect/Seq/SeqPasses.h"
// #include "circt/Support/LoweringOptions.h"
// #include "circt/Support/LoweringOptionsParser.h"
// #include "circt/Support/Version.h"
// #include "circt/Transforms/Passes.h"
// #include "mlir/Bytecode/BytecodeReader.h"
// #include "mlir/Bytecode/BytecodeWriter.h"
// #include "mlir/Dialect/Func/IR/FuncOps.h"
// #include "mlir/IR/AsmState.h"
// #include "mlir/IR/BuiltinOps.h"
// #include "mlir/Parser/Parser.h"
// #include "mlir/Pass/Pass.h"
// #include "mlir/Pass/PassInstrumentation.h"
// #include "mlir/Pass/PassManager.h"
// #include "mlir/Support/FileUtilities.h"
// #include "mlir/Support/Timing.h"
// #include "mlir/Support/ToolUtilities.h"
// #include "mlir/Transforms/GreedyPatternRewriteDriver.h"
#include "mlir/Transforms/Passes.h"
// #include "llvm/Support/Chrono.h"
// #include "llvm/Support/CommandLine.h"
// #include "llvm/Support/FileSystem.h"
// #include "llvm/Support/InitLLVM.h"
// #include "llvm/Support/Path.h"
// #include "llvm/Support/PrettyStackTrace.h"
// #include "llvm/Support/SourceMgr.h"
// #include "llvm/Support/ToolOutputFile.h"


using namespace llvm;
using namespace mlir;
using namespace circt;

MlirAttribute mlirIntegerAttrGetFromString(MlirType type, MlirStringRef value) {
  auto intType = unwrap(type).cast<IntegerType>();
  auto intWidth = intType.getWidth();
  auto valueStr = unwrap(value);
  auto tmpWidth = std::max<size_t>(intWidth, (valueStr.size() - 1) * 64 / 22);
  return wrap(
      IntegerAttr::get(intType, APInt(tmpWidth, valueStr, 10).trunc(intWidth)));
}

bool mlirLocationIsFileLineCol(MlirLocation loc) {
  return unwrap(loc).isa<FileLineColLoc>();
}

MlirStringRef mlirFileLineColLocGetFilename(MlirLocation loc) {
  return wrap(unwrap(loc).cast<FileLineColLoc>().getFilename().strref());
}

unsigned mlirFileLineColLocGetLine(MlirLocation loc) {
  return unwrap(loc).cast<FileLineColLoc>().getLine();
}

unsigned mlirFileLineColLocGetColumn(MlirLocation loc) {
  return unwrap(loc).cast<FileLineColLoc>().getColumn();
}


void registerFirrtlPasses() {
    // MLIR transforms:
    // Don't use registerTransformsPasses, pulls in too much.
    registerCSEPass();
    registerCanonicalizerPass();
    registerStripDebugInfoPass();
    registerSymbolDCEPass();

    // Dialect passes:
    firrtl::registerPasses();
    sv::registerPasses();

    // Export passes:
    registerExportChiselInterfacePass();
    registerExportSplitChiselInterfacePass();
    registerExportSplitVerilogPass();
    registerExportVerilogPass();
}