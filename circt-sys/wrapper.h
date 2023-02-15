#ifndef __C_WRAPPER__H__
#define __C_WRAPPER__H__

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
#include "mlir-c/Interfaces.h"
#include "mlir-c/Diagnostics.h"
#include "mlir-c/Dialect/SCF.h"
#include "mlir-c/Dialect/ControlFlow.h"
#include "mlir-c/Dialect/Shape.h"
#include "mlir-c/Dialect/Func.h"
#include "mlir-c/Dialect/LLVM.h"
#include "mlir-c/Dialect/Transform.h"
// #include "mlir-c/Dialect/Async.h"
// #include "mlir-c/Dialect/SparseTensor.h"
// #include "mlir-c/Dialect/Linalg.h"

#include "circt-c/ExportVerilog.h"
#include "circt-c/Dialect/Comb.h"
#include "circt-c/Dialect/Seq.h"
#include "circt-c/Dialect/HW.h"
#include "circt-c/Dialect/HWArith.h"
#include "circt-c/Dialect/SV.h"
#include "circt-c/Dialect/FSM.h"
#include "circt-c/Dialect/Handshake.h"
#include "circt-c/Dialect/FIRRTL.h"
#include "circt-c/Dialect/ESI.h"
// #include "circt-c/Dialect/LLHD.h"
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


MLIR_CAPI_EXPORTED void registerHWPasses();
MLIR_CAPI_EXPORTED void registerFIRRTLPasses();
MLIR_CAPI_EXPORTED void circtRegisterTransformsPasses();

MLIR_CAPI_EXPORTED void firrtlRegisterLowerCHIRRTLPass();
MLIR_CAPI_EXPORTED MlirPass firrtlCreateLowerCHIRRTLPass();

MLIR_CAPI_EXPORTED MlirPass seqCreateSeqLowerToSVPass();

MLIR_CAPI_EXPORTED MlirPass seqCreateSeqLowerSeqHLMemPass();

MLIR_CAPI_EXPORTED MlirPass seqCreateSeqFIRRTLLowerToSVPass();

MLIR_CAPI_EXPORTED MlirPass mlirCreateTransformsViewOpGraph();

MLIR_CAPI_EXPORTED MlirPass hwCreateFlattenIOPass();
MLIR_CAPI_EXPORTED void hwRegisterFlattenIOPass();

MLIR_CAPI_EXPORTED MlirPass hwCreateHWSpecializePass();
MLIR_CAPI_EXPORTED void hwRegisterHWSpecializePass();


#ifdef __cplusplus
}
#endif

#endif // __C_WRAPPER__H__