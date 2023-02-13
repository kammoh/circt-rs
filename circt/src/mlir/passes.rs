use circt_sys::*;

wrap_raw_ptr!(Pass);

// pub struct Pass(mlir::Pass);


impl Pass {
    // pub fn create_cse() {
    //     let a = unsafe { mlir::createCSEPass()};
    // }
//     pub fn create_cse() {
//         unsafe{circt_sys::mlir::createCSEPass()};
//     }

//     pub fn create_canonicalizer() -> Pass {
//         Pass::try_from_raw(unsafe { mlirCreateTransformsCanonicalizer() }).unwrap()
//     }

//     pub fn create_flowsink() -> Pass {
//         Pass::try_from_raw(unsafe { mlirCreateTransformsControlFlowSink() }).unwrap()
//     }

//     pub fn create_inliner() -> Pass {
//         Pass::try_from_raw(unsafe { mlirCreateTransformsInliner() }).unwrap()
//     }

//     pub fn create_view_op_graph() -> Pass {
//         Pass::try_from_raw(unsafe { mlirCreateTransformsViewOpGraph() }).unwrap()
//     }
}

// pub fn register_all_passes() {
//     unsafe { mlirRegisterAllPasses() }
// }

// pub fn register_conversion_passes() {
//     unsafe { mlirRegisterConversionPasses() }
// }
