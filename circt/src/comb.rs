// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;

define_dialect!(comb);

/// Predicate for a comparison operation.
#[derive(PartialEq, Eq)]
pub enum CmpPred {
    Eq,
    Neq,
    Slt,
    Sle,
    Sgt,
    Sge,
    Ult,
    Ule,
    Ugt,
    Uge,
}

def_operation_single_result!(AndOp, "comb.and");
def_operation_single_result!(OrOp, "comb.or");
def_operation_single_result!(XorOp, "comb.xor");

def_simple_binary_operation!(AddOp, "comb.add");
def_simple_binary_operation!(SubOp, "comb.sub");
def_simple_binary_operation!(MulOp, "comb.mul");
def_simple_binary_operation!(DivUOp, "comb.divu");
def_simple_binary_operation!(DivSOp, "comb.divs");
def_simple_binary_operation!(ModUOp, "comb.modu");
def_simple_binary_operation!(ModSOp, "comb.mods");
def_simple_binary_operation!(ShlOp, "comb.shl");
def_simple_binary_operation!(ShrUOp, "comb.shru");
def_simple_binary_operation!(ShrSOp, "comb.shrs");
def_operation_single_result!(ICmpOp, "comb.icmp");
def_operation_single_result!(MuxOp, "comb.mux");
def_operation_single_result!(ExtractOp, "comb.extract");
def_operation_single_result!(ConcatOp, "comb.concat");

// impl ICmpOp {
//     /// Create a new comparison operation.
//     pub fn new(builder: &mut OpBuilder, pred: CmpPred, lhs: Value, rhs: Value) -> Self {
//         builder.build_with(|builder, state| {
//             state.add_operand(lhs);
//             state.add_operand(rhs);
//             let attr_ty = builder.ctx.get_integer_type(64).unwrap();
//             let attr = get_integer_attr(&attr_ty, pred as _);
//             state.add_attribute("predicate", &attr);
//             state.add_result(builder.ctx.get_integer_type(1).unwrap());
//         })
//     }
// }

// impl MuxOp {
//     /// Create a new mux operation.
//     pub fn new(
//         builder: &mut OpBuilder,
//         cond: Value,
//         true_value: Value,
//         false_value: Value,
//     ) -> Self {
//         builder.build_with(|_, state| {
//             state.add_operand(cond);
//             state.add_operand(true_value);
//             state.add_operand(false_value);
//             state.add_result(true_value.ty().unwrap());
//         })
//     }
// }

// impl ExtractOp {
//     /// Extract a bit range from an integer.
//     pub fn new(builder: &mut OpBuilder, ty: Type, value: Value, offset: usize) -> Self {
//         builder.build_with(|builder, state| {
//             state.add_operand(value);
//             let attr = get_integer_attr(&builder.ctx.get_integer_type(32).unwrap(), offset as _);
//             state.add_attribute("lowBit", &attr);
//             state.add_result(ty);
//         })
//     }

//     pub fn with_sizes(builder: &mut OpBuilder, value: Value, offset: usize, length: usize) -> Self {
//         Self::new(
//             builder,
//             builder.ctx.get_integer_type(length).unwrap(),
//             value,
//             offset,
//         )
//     }
// }

// impl ConcatOp {
//     pub fn new(builder: &mut OpBuilder, values: impl IntoIterator<Item = Value>) -> Self {
//         builder.build_with(|builder, state| {
//             let mut width = 0;
//             for value in values {
//                 state.add_operand(value);
//                 width += integer_type_width(value.ty().unwrap());
//             }
//             state.add_result(builder.ctx.get_integer_type(width).unwrap());
//         })
//     }
// }

// impl ShrUOp {
//     pub fn with_sizes(builder: &mut OpBuilder, value: Value, amount: Value) -> Self {
//         let amount = trunc_or_zext(builder, amount, value.ty().unwrap());
//         ShrUOp::new(builder, value, amount)
//     }
// }

// impl ShrSOp {
//     pub fn with_sizes(builder: &mut OpBuilder, value: Value, amount: Value) -> Self {
//         let amount = trunc_or_zext(builder, amount, value.ty().unwrap());
//         ShrSOp::new(builder, value, amount)
//     }
// }

// impl ShlOp {
//     pub fn with_sizes(builder: &mut OpBuilder, value: Value, amount: Value) -> Self {
//         let amount = trunc_or_zext(builder, amount, value.ty().unwrap());
//         ShlOp::new(builder, value, amount)
//     }
// }

// pub(crate) fn clog2(value: usize) -> usize {
//     usize::BITS as usize - value.next_power_of_two().leading_zeros() as usize - 1
// }

// pub(crate) fn type_width(ty: Type) -> usize {
//     if is_array_type(ty) {
//         array_type_size(ty)
//     } else if is_integer_type(ty) {
//         integer_type_width(ty)
//     } else {
//         panic!("unsupported indexing target type {}", ty)
//     }
// }

// pub(crate) fn type_clog2(ty: Type) -> usize {
//     clog2(type_width(ty))
// }

// pub(crate) fn trunc_or_zext_to_clog2(
//     builder: &mut OpBuilder,
//     index: Value,
//     into_ty: Type,
// ) -> Value {
//     let target_width = std::cmp::max(type_clog2(into_ty), 1);
//     trunc_or_zext(
//         builder,
//         index,
//         builder.ctx.get_integer_type(target_width).unwrap(),
//     )
// }

// pub(crate) fn trunc_or_zext(builder: &mut OpBuilder, index: Value, into_ty: Type) -> Value {
//     let target_width = type_width(into_ty);
//     let actual_width = integer_type_width(index.ty().unwrap());
//     if target_width < actual_width {
//         ExtractOp::with_sizes(builder, index, 0, target_width).result(0).unwrap()
//     } else if target_width > actual_width {
//         let zero = ConstantOp::new(builder, target_width - actual_width, &BigInt::zero()).result(0).unwrap();
//         ConcatOp::new(builder, [zero, index.clone()].iter().cloned()).result(0).unwrap()
//     } else {
//         index.clone()
//     }
// }
