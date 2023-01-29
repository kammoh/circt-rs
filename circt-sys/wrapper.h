#ifndef __C_WRAPPER__H__
#define __C_WRAPPER__H__

#include "llvm-c/Error.h"
#include "llvm-c/Core.h"
#include "llvm-c/ExecutionEngine.h"
#include "llvm-c/Disassembler.h"
#include "llvm-c/ExternC.h"
#include "llvm-c/Types.h"
#include "llvm-c/BitWriter.h"
#include "llvm-c/OrcEE.h"
#include "llvm-c/LLJIT.h"
#include "llvm-c/DebugInfo.h"
#include "llvm-c/IRReader.h"
#include "llvm-c/DataTypes.h"
#include "llvm-c/Transforms/Utils.h"
#include "llvm-c/Transforms/PassManagerBuilder.h"
#include "llvm-c/Transforms/PassBuilder.h"
#include "llvm-c/Transforms/Scalar.h"
#include "llvm-c/Transforms/Vectorize.h"
#include "llvm-c/Transforms/InstCombine.h"
#include "llvm-c/Transforms/IPO.h"
#include "llvm-c/Initialization.h"
#include "llvm-c/TargetMachine.h"
#include "llvm-c/Comdat.h"
#include "llvm-c/Remarks.h"
#include "llvm-c/Target.h"
#include "llvm-c/Object.h"
#include "llvm-c/Orc.h"
#include "llvm-c/Linker.h"
#include "llvm-c/ErrorHandling.h"
#include "llvm-c/DisassemblerTypes.h"
#include "llvm-c/BitReader.h"
#include "llvm-c/Analysis.h"
#include "llvm-c/lto.h"
#include "llvm-c/Support.h"

#include "mlir-c/IntegerSet.h"
#include "mlir-c/BuiltinTypes.h"
#include "mlir-c/Debug.h"
#include "mlir-c/ExecutionEngine.h"
#include "mlir-c/IR.h"
#include "mlir-c/Pass.h"
#include "mlir-c/AffineExpr.h"
#include "mlir-c/AffineMap.h"
#include "mlir-c/BuiltinAttributes.h"
#include "mlir-c/Diagnostics.h"
#include "mlir-c/RegisterEverything.h"
#include "mlir-c/Conversion.h"
#include "mlir-c/Support.h"
#include "mlir-c/Transforms.h"
#include "mlir-c/Diagnostics.h"
#include "mlir-c/Dialect/SCF.h"
#include "mlir-c/Dialect/ControlFlow.h"
#include "mlir-c/Dialect/Shape.h"
#include "mlir-c/Dialect/Func.h"
#include "mlir-c/Dialect/LLVM.h"
#include "mlir-c/Dialect/Async.h"
#include "mlir-c/Dialect/SparseTensor.h"
#include "mlir-c/Dialect/Linalg.h"
#include "mlir-c/Dialect/Transform.h"

#include "circt-c/ExportVerilog.h"
#include "circt-c/Dialect/Comb.h"
#include "circt-c/Dialect/Seq.h"
#include "circt-c/Dialect/HW.h"
#include "circt-c/Dialect/HWArith.h"
#include "circt-c/Dialect/SV.h"
#include "circt-c/Dialect/FSM.h"
#include "circt-c/Dialect/Handshake.h"
#include "circt-c/Dialect/FIRRTL.h"
// #include "circt-c/Dialect/LLHD.h"
// #include "circt-c/Dialect/ESI.h"
// #include "circt-c/Dialect/MSFT.h"
// #include "circt-c/Dialect/Moore.h"


#ifdef __cplusplus
extern "C" {
#endif

/// Creates an integer attribute of the given type by parsing the given string
/// into an integer value.
MLIR_CAPI_EXPORTED MlirAttribute
mlirIntegerAttrGetFromString(MlirType type, MlirStringRef value);

//===----------------------------------------------------------------------===//
// Location API Extensions
//===----------------------------------------------------------------------===//

MLIR_CAPI_EXPORTED bool mlirLocationIsFileLineCol(MlirLocation);
MLIR_CAPI_EXPORTED MlirStringRef mlirFileLineColLocGetFilename(MlirLocation);
MLIR_CAPI_EXPORTED unsigned mlirFileLineColLocGetLine(MlirLocation);
MLIR_CAPI_EXPORTED unsigned mlirFileLineColLocGetColumn(MlirLocation);

#ifdef __cplusplus
}
#endif

#endif // __C_WRAPPER__H__