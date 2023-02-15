use crate::crate_prelude::*;
use circt_sys::*;

wrap_raw_ptr!(Pass);

impl Pass {}

pub fn register_all_passes() {
    unsafe { mlirRegisterAllPasses() }
}

pub fn register_conversion_passes() {
    unsafe { mlirRegisterConversionPasses() }
}

pub fn register_passes() {
    unsafe { mlirRegisterTransformsPasses() }
}

pub fn register_cse() {
    unsafe { mlirRegisterTransformsCSE() }
}

pub fn register_cf_sink() {
    unsafe { mlirRegisterTransformsControlFlowSink() }
}

pub fn register_inliner() {
    unsafe { mlirRegisterTransformsInliner() }
}

pub fn register_view_op_graph() {
    unsafe { mlirRegisterTransformsViewOpGraph() }
}

pub fn register_canonicalizer() {
    unsafe { mlirRegisterTransformsCanonicalizer() }
}

pub fn cse() -> Pass {
    Pass::try_from_raw(unsafe { mlirCreateTransformsCSE() }).unwrap()
}

pub fn canonicalize() -> Pass {
    Pass::try_from_raw(unsafe { mlirCreateTransformsCanonicalizer() }).unwrap()
}

pub fn cf_sink() -> Pass {
    Pass::try_from_raw(unsafe { mlirCreateTransformsControlFlowSink() }).unwrap()
}

pub fn inliner() -> Pass {
    Pass::try_from_raw(unsafe { mlirCreateTransformsInliner() }).unwrap()
}

pub fn view_op_graph() -> Pass {
    Pass::try_from_raw(unsafe { mlirCreateTransformsViewOpGraph() }).unwrap()
}
