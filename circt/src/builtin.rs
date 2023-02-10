// Copyright (c) 2016-2021 Fabian Schuiki

//! mlir Operations

use crate::crate_prelude::*;

def_operation!(ModuleOp, "builtin.module");

impl SingleRegionOp for ModuleOp {}

impl SingleBlockOp for ModuleOp {}

impl ModuleOp {
    pub fn build(builder: &mut OpBuilder) -> Option<Self> {
        let region = Region::new();
        let block = Block::new();
        builder.build_with(|_, state| {
            region.append_block(&block);
            state.add_region(&region)
        })
    }
}

// def_operation!(
//     UnrealizedConversionCastOp,
//     "builtin.unrealized_conversion_cast"
// );

// impl UnrealizedConversionCastOp {
//     /// Create a new unrealized conversion cast operation.
//     pub fn new(
//         builder: &mut OpBuilder,
//         values: impl IntoIterator<Item = Value>,
//         result_tys: impl IntoIterator<Item = Type>,
//     ) -> Self {
//         builder.build_with(|_, state| {
//             for value in values {
//                 state.add_operand(value);
//             }
//             for rty in result_tys {
//                 state.add_result(rty);
//             }
//         })
//     }
// }
