use crate::crate_prelude::*;
use circt_sys::*;

// pub fn register_passes() {
//     unsafe { mlirRegisterTransformsPasses() }
// }

pub fn register_cse() {
    unsafe {cxx_bindings::mlir::registerCSE() }
   
}

pub fn register_canonicalizer() {
    unsafe { cxx_bindings::mlir::registerCanonicalizer() }
}

// pub fn register_flowsink() {
//     unsafe { mlirRegisterTransformsControlFlowSink() }
// }

// pub fn register_inliner() {
//     unsafe { mlirRegisterTransformsInliner() }
// }

// pub fn register_view_op_graph() {
//     unsafe { mlirRegisterTransformsViewOpGraph() }
// }
