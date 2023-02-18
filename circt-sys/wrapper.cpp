#include "wrapper.h"
#include "mlir/CAPI/IR.h"
#include "mlir/CAPI/Support.h"
#include "mlir/CAPI/Pass.h"
#include "mlir/CAPI/Registration.h"
#include "mlir/Transforms/Passes.h"

#include "circt/Transforms/Passes.h"
#include "circt/Conversion/Passes.h"
#include "circt/InitAllPasses.h"
#include "circt/Support/LoweringOptions.h"
#include "circt/Support/Version.h"
#include "circt/Dialect/Pipeline/Pipeline.h"
#include "circt/Dialect/HW/HWPasses.h"
#include "circt/Dialect/HW/HWDialect.h"
#include "circt/Dialect/HWArith/HWArithDialect.h"
#include "circt/Dialect/Comb/CombDialect.h"
#include "circt/Dialect/Seq/SeqPasses.h"
#include "circt/Dialect/Seq/SeqDialect.h"
#include "circt/Dialect/FSM/FSMPasses.h"
#include "circt/Dialect/FSM/FSMOps.h"
#include "circt/Dialect/Pipeline/PipelinePasses.h"
#include "circt/Dialect/Handshake/HandshakeDialect.h"
#include "circt/Dialect/Interop/InteropDialect.h"
#include "circt/Dialect/ESI/ESIDialect.h"
#include "circt/Dialect/FIRRTL/Passes.h"
#include "circt/Dialect/FIRRTL/CHIRRTLDialect.h"
#include "circt/Dialect/FIRRTL/FIRRTLDialect.h"
#include "circt/Dialect/Handshake/HandshakePasses.h"
#include "circt/Dialect/SV/SVPasses.h"
#include "circt/Dialect/SV/SVDialect.h"
// #include "circt/Conversion/ExportVerilog.h"

using namespace llvm;
using namespace mlir;
using namespace circt;

MlirAttribute mlirIntegerAttrGetFromString(MlirType type, MlirStringRef value) {
    auto intType = unwrap(type).cast<IntegerType>();
    auto intWidth = intType.getWidth();
    auto valueStr = unwrap(value);
    auto tmpWidth = std::max<size_t>(intWidth, (valueStr.size() - 1) * 64 / 22);
    return wrap(IntegerAttr::get(
        intType, APInt(tmpWidth, valueStr, 10).trunc(intWidth)));
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

void mlirRegisterTransformsPasses() {
  mlir::registerTransformsPasses();
}

void circtRegisterTransformsPasses() {
  circt::registerTransformsPasses();
}

MlirPass mlirCreateTransformsCSE() {
  return wrap(mlir::createCSEPass().release());
}
void mlirRegisterTransformsCSE() {
  mlir::registerCSEPass();
}

MlirPass seqCreateSeqLowerToSVPass() {
  return wrap(circt::seq::createSeqLowerToSVPass().release());
}

MlirPass seqCreateSeqLowerSeqHLMemPass() {
  return wrap(circt::seq::createLowerSeqHLMemPass().release());
}
MlirPass seqCreateSeqFIRRTLLowerToSVPass() {
  return wrap(circt::seq::createSeqFIRRTLLowerToSVPass().release());
}

MlirPass mlirCreateTransformsCanonicalizer() {
  return wrap(mlir::createCanonicalizerPass().release());
}
void mlirRegisterTransformsCanonicalizer() {
  mlir::registerCanonicalizer();
}


MlirPass mlirCreateTransformsControlFlowSink() {
  return wrap(::mlir::createControlFlowSinkPass().release());
}
void mlirRegisterTransformsControlFlowSink() {
  mlir::registerControlFlowSink();
}


MlirPass mlirCreateTransformsGenerateRuntimeVerification() {
  return wrap(mlir::createGenerateRuntimeVerificationPass().release());
}
void mlirRegisterTransformsGenerateRuntimeVerification() {
  mlir::registerGenerateRuntimeVerification();
}


MlirPass mlirCreateTransformsInliner() {
  return wrap(mlir::createInlinerPass().release());
}
void mlirRegisterTransformsInliner() {
  mlir::registerInliner();
}


MlirPass mlirCreateTransformsLocationSnapshot() {
  return wrap(mlir::createLocationSnapshotPass().release());
}
void mlirRegisterTransformsLocationSnapshot() {
  mlir::registerLocationSnapshot();
}


MlirPass mlirCreateTransformsLoopInvariantCodeMotion() {
  return wrap(mlir::createLoopInvariantCodeMotionPass().release());
}
void mlirRegisterTransformsLoopInvariantCodeMotion() {
  mlir::registerLoopInvariantCodeMotion();
}


MlirPass mlirCreateTransformsPrintOpStats() {
  return wrap(mlir::createPrintOpStatsPass().release());
}
void mlirRegisterTransformsPrintOpStats() {
  mlir::registerPrintOpStats();
}


MlirPass mlirCreateTransformsSCCP() {
  return wrap(mlir::createSCCPPass().release());
}

void mlirRegisterTransformsSCCP() {
  mlir::registerSCCP();
}


MlirPass mlirCreateTransformsStripDebugInfo() {
  return wrap(mlir::createStripDebugInfoPass().release());
}
void mlirRegisterTransformsStripDebugInfo() {
  registerStripDebugInfo();
}

MlirPass mlirCreateTransformsSymbolDCE() {
  return wrap(mlir::createSymbolDCEPass().release());
}
void mlirRegisterTransformsSymbolDCE() {
  mlir::registerSymbolDCE();
}


MlirPass mlirCreateTransformsSymbolPrivatize() {
  return wrap(mlir::createSymbolPrivatizePass().release());
}
void mlirRegisterTransformsSymbolPrivatize() {
  mlir::registerSymbolPrivatize();
}


MlirPass mlirCreateTransformsTopologicalSort() {
  return wrap(mlir::createTopologicalSortPass().release());
}
void mlirRegisterTransformsTopologicalSort() {
  mlir::registerTopologicalSort();
}


MlirPass mlirCreateTransformsViewOpGraph() {
  return wrap(mlir::createPrintOpGraphPass().release());
}

void mlirRegisterTransformsViewOpGraph() {
  mlir::registerViewOpGraph();
}


void registerHWPasses() {
  circt::hw::registerPasses();
}

void registerFIRRTLPasses() {
  circt::firrtl::registerPasses();
}

MlirPass firrtlCreateLowerCHIRRTLPass() {
  return wrap(circt::firrtl::createLowerCHIRRTLPass().release());
}

void firrtlRegisterLowerCHIRRTLPass() {
    ::mlir::registerPass([]() -> std::unique_ptr<::mlir::Pass> {
    return ::circt::firrtl::createLowerCHIRRTLPass();
  });
}

MlirPass hwCreateFlattenIOPass() {
  return wrap(::circt::hw::createFlattenIOPass().release());
}

void hwRegisterFlattenIOPass() {
    ::mlir::registerPass([]() -> std::unique_ptr<::mlir::Pass> {
    return ::circt::hw::createFlattenIOPass();
  });
}

MlirPass hwCreateHWSpecializePass() {
  return wrap(::circt::hw::createHWSpecializePass().release());
}

void hwRegisterHWSpecializePass() {
    ::mlir::registerPass([]() -> std::unique_ptr<::mlir::Pass> {
    return ::circt::hw::createHWSpecializePass();
  });
}
