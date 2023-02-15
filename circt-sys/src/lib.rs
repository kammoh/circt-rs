#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_variables,
    unused
)]

pub use libc::size_t;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings/bindings.rs"));

/*
use autocxx::prelude::*;

include_cpp! {
    #include "cxx_wrapper.hpp"
    // generate!("llvm::TrackingStatistic")
    // generate!("llvm::NoopStatistic")
    // generate_pod!("mlir::StorageUniquer")
    // generate_pod!("mlir::DiagnosticEngine")
    // generate_pod!("mlir::Location")
    // generate_pod!("mlir::MLIRContext")
    // generate!("mlir::Dialect")
    // // generate!("mlir::Pass")
    // generate!("mlir::OpOperand")
    // generate!("mlir::Type")
    // generate!("mlir::Attribute")
    // generate!("mlir::AffineMapAttr")
    // generate!("mlir::ArrayAttr")
    // generate!("mlir::DenseArrayAttr")
    // generate!("mlir::DenseIntOrFPElementsAttr")
    // generate!("mlir::DenseResourceElementsAttr")
    // generate!("mlir::DenseStringElementsAttr")
    // generate!("mlir::DictionaryAttr")
    // generate!("mlir::FloatAttr")
    // generate!("mlir::IntegerAttr")
    // generate!("mlir::IntegerSetAttr")
    // generate!("mlir::OpaqueAttr")
    // generate!("mlir::StringAttr")
    // generate!("mlir::SymbolRefAttr")
    // generate!("mlir::TypeAttr")
    // generate!("mlir::UnitAttr")
    // generate!("mlir::StridedLayoutAttr")
    // generate!("mlir::Value")
    // block!("llvm::iterator_range")
    // block!("std::reverse_iterator")
    // // block!("mlir::ValueUserIterator")
    // // block!("mlir::ValueUseIterator")
    // generate_pod!("mlir::BlockArgument")
    // generate_pod!("circt::hw::AggregateConstantOp")
    // generate_pod!("circt::hw::ArrayConcatOp")
    // generate_pod!("circt::hw::ArrayGetOp")
    // generate_pod!("circt::hw::ArraySliceOp")
    // generate_pod!("circt::hw::BitcastOp")
    // generate_pod!("circt::hw::ConstantOp")
    // generate_pod!("circt::hw::EnumConstantOp")
    // generate_pod!("circt::hw::GlobalRefOp")
    // generate_pod!("circt::hw::HWGeneratorSchemaOp")
    // generate_pod!("circt::hw::HWModuleExternOp")
    // generate_pod!("circt::hw::HWModuleGeneratedOp")
    // generate_pod!("circt::hw::HWModuleOp")
    // generate_pod!("circt::hw::HierPathOp")
    // generate_pod!("circt::hw::InstanceOp")
    // generate_pod!("circt::hw::OutputOp")
    // generate_pod!("circt::hw::ParamValueOp")
    // generate_pod!("circt::hw::ProbeOp")
    // generate_pod!("circt::hw::StructCreateOp")
    // generate_pod!("circt::hw::StructExplodeOp")
    // generate_pod!("circt::hw::StructExtractOp")
    // generate_pod!("circt::hw::StructInjectOp")
    // generate_pod!("circt::hw::TypeScopeOp")
    // generate_pod!("circt::hw::TypedeclOp")
    // generate_pod!("circt::hw::UnionCreateOp")
    // generate_pod!("circt::hw::UnionExtractOp")
    // // generate!("mlir::Operation")
    // // generate!("mlir::Region")
    // // block!("mlir::Region::cloneInto")
    // // block!("llvm::iplist_impl")
    // // block!("ilist_iterator")
    // // block!("llvm::ilist_iterator")
    // // block!("mlir::ilist_iterator")
    // // block!("mlir::Region::iterator")
    // // generate!("mlir::Block")
    // generate_ns!("circt::comb")
    // block!("circt::comb::AddOpGenericAdaptor")
    // block!("circt::comb::AndOpGenericAdaptor")
    // block!("circt::comb::ConcatOpGenericAdaptor")
    // block!("circt::comb::DivSOpGenericAdaptor")
    // block!("circt::comb::DivUOpGenericAdaptor")
    // block!("circt::comb::ExtractOpGenericAdaptor")
    // block!("circt::comb::ICmpOpGenericAdaptor")
    // block!("circt::comb::ModSOpGenericAdaptor")
    // block!("circt::comb::ModUOpGenericAdaptor")
    // block!("circt::comb::MulOpGenericAdaptor")
    // block!("circt::comb::MuxOpGenericAdaptor")
    // block!("circt::comb::OrOpGenericAdaptor")
    // block!("circt::comb::ParityOpGenericAdaptor")
    // block!("circt::comb::ReplicateOpGenericAdaptor")
    // block!("circt::comb::ShlOpGenericAdaptor")
    // block!("circt::comb::ShrSOpGenericAdaptor")
    // block!("circt::comb::ShrUOpGenericAdaptor")
    // block!("circt::comb::SubOpGenericAdaptor")
    // block!("circt::comb::XorOpGenericAdaptor")
    // // generate!("circt::seq::CompRegClockEnabledOp")
    // generate!("circt::seq::CompRegOp")
    // generate!("circt::seq::FirRegOp")
    // generate!("circt::seq::HLMemOp")
    // generate!("circt::seq::ReadPortOp")
    // generate!("circt::seq::WritePortOp")
    generate!("mlir::registerCSE")
    generate!("mlir::registerCanonicalizer")
    generate!("mlir::registerControlFlowSink")
    generate!("mlir::registerStripDebugInfoPass")
    generate!("mlir::registerSymbolDCEPass")
    generate!("circt::seq::registerPasses")
    generate!("circt::hw::registerPasses")
    generate!("circt::sv::registerPasses")
    generate!("circt::firrtl::registerPasses")
    generate!("circt::registerExportVerilog")
    generate!("circt::registerExportSplitVerilog")
    generate!("mlir::registerGenerateRuntimeVerification")
    generate!("mlir::registerInliner")
    generate!("mlir::registerPrintOpStats")
    generate!("mlir::registerSymbolDCE")
}


pub mod cxx_bindings {
    pub use super::ffi::*;
}
*/

#[cfg(test)]
mod tests {}
