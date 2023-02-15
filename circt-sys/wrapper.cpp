#include "wrapper.h"
#include "mlir/CAPI/IR.h"
#include "mlir/CAPI/Support.h"
#include "mlir/CAPI/Pass.h"
#include "mlir/CAPI/Registration.h"
#include "mlir/Transforms/Passes.h"
#include "mlir/Transforms/ViewOpGraph.h"
#include "circt/Transforms/Passes.h"
#include "circt/Conversion/Passes.h"
#include "circt/Dialect/FIRRTL/Passes.h"
#include "circt/Dialect/Seq/SeqPasses.h"
#include "circt/Dialect/HW/HWPasses.h"
#include "circt/Dialect/Pipeline/PipelinePasses.h"
#include "circt/Dialect/Handshake/HandshakePasses.h"
#include "circt/Dialect/FSM/FSMPasses.h"
#include "circt/Dialect/SV/SVPasses.h"
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
  registerCSE();
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
  registerCanonicalizer();
}


MlirPass mlirCreateTransformsControlFlowSink() {
  return wrap(::mlir::createControlFlowSinkPass().release());
}
void mlirRegisterTransformsControlFlowSink() {
  registerControlFlowSink();
}


MlirPass mlirCreateTransformsGenerateRuntimeVerification() {
  return wrap(mlir::createGenerateRuntimeVerificationPass().release());
}
void mlirRegisterTransformsGenerateRuntimeVerification() {
  registerGenerateRuntimeVerification();
}


MlirPass mlirCreateTransformsInliner() {
  return wrap(mlir::createInlinerPass().release());
}
void mlirRegisterTransformsInliner() {
  registerInliner();
}


MlirPass mlirCreateTransformsLocationSnapshot() {
  return wrap(mlir::createLocationSnapshotPass().release());
}
void mlirRegisterTransformsLocationSnapshot() {
  registerLocationSnapshot();
}


MlirPass mlirCreateTransformsLoopInvariantCodeMotion() {
  return wrap(mlir::createLoopInvariantCodeMotionPass().release());
}
void mlirRegisterTransformsLoopInvariantCodeMotion() {
  registerLoopInvariantCodeMotion();
}


MlirPass mlirCreateTransformsPrintOpStats() {
  return wrap(mlir::createPrintOpStatsPass().release());
}
void mlirRegisterTransformsPrintOpStats() {
  registerPrintOpStats();
}


MlirPass mlirCreateTransformsSCCP() {
  return wrap(mlir::createSCCPPass().release());
}

void mlirRegisterTransformsSCCP() {
  registerSCCP();
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
  registerSymbolDCE();
}


MlirPass mlirCreateTransformsSymbolPrivatize() {
  return wrap(mlir::createSymbolPrivatizePass().release());
}
void mlirRegisterTransformsSymbolPrivatize() {
  registerSymbolPrivatize();
}


MlirPass mlirCreateTransformsTopologicalSort() {
  return wrap(mlir::createTopologicalSortPass().release());
}
void mlirRegisterTransformsTopologicalSort() {
  registerTopologicalSort();
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
