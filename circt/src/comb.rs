// Copyright (c) 2016-2021 Fabian Schuiki

use circt_sys::capi::mlirGetDialectHandle__comb__;

use crate::crate_prelude::*;
use crate::hw::{array_type_size, is_array_type, ConstantOp};

pub fn dialect() -> DialectHandle {
    DialectHandle::from_raw(unsafe { mlirGetDialectHandle__comb__() })
}

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

def_simple_binary_operation!(AndOp, "comb.and");
def_simple_binary_operation!(OrOp, "comb.or");
def_simple_binary_operation!(XorOp, "comb.xor");
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

impl ICmpOp {
    /// Create a new comparison operation.
    pub fn new(builder: &mut Builder, pred: CmpPred, lhs: Value, rhs: Value) -> Self {
        builder.build_with(|builder, state| {
            state.add_operand(lhs);
            state.add_operand(rhs);
            let attr_ty = get_integer_type(builder.cx, 64);
            let attr = get_integer_attr(
                attr_ty,
                match pred {
                    CmpPred::Eq => 0,
                    CmpPred::Neq => 1,
                    CmpPred::Slt => 2,
                    CmpPred::Sle => 3,
                    CmpPred::Sgt => 4,
                    CmpPred::Sge => 5,
                    CmpPred::Ult => 6,
                    CmpPred::Ule => 7,
                    CmpPred::Ugt => 8,
                    CmpPred::Uge => 9,
                },
            );
            state.add_attribute("predicate", attr);
            state.add_result(get_integer_type(builder.cx, 1));
        })
    }
}

impl MuxOp {
    /// Create a new mux operation.
    pub fn new(builder: &mut Builder, cond: Value, true_value: Value, false_value: Value) -> Self {
        builder.build_with(|_, state| {
            state.add_operand(cond);
            state.add_operand(true_value);
            state.add_operand(false_value);
            state.add_result(true_value.ty());
        })
    }
}

impl ExtractOp {
    /// Extract a bit range from an integer.
    pub fn new(builder: &mut Builder, ty: Type, value: Value, offset: usize) -> Self {
        builder.build_with(|builder, state| {
            state.add_operand(value);
            let attr = get_integer_attr(get_integer_type(builder.cx, 32), offset as _);
            state.add_attribute("lowBit", attr);
            state.add_result(ty);
        })
    }

    pub fn with_sizes(builder: &mut Builder, value: Value, offset: usize, length: usize) -> Self {
        Self::new(builder, get_integer_type(builder.cx, length), value, offset)
    }
}

impl ConcatOp {
    pub fn new(builder: &mut Builder, values: impl IntoIterator<Item = Value>) -> Self {
        builder.build_with(|builder, state| {
            let mut width = 0;
            for value in values {
                state.add_operand(value);
                width += integer_type_width(value.ty());
            }
            state.add_result(get_integer_type(builder.cx, width));
        })
    }
}

impl ShrUOp {
    pub fn with_sizes(builder: &mut Builder, value: Value, amount: Value) -> Self {
        let amount = trunc_or_zext(builder, amount, value.ty());
        ShrUOp::new(builder, value, amount)
    }
}

impl ShrSOp {
    pub fn with_sizes(builder: &mut Builder, value: Value, amount: Value) -> Self {
        let amount = trunc_or_zext(builder, amount, value.ty());
        ShrSOp::new(builder, value, amount)
    }
}

impl ShlOp {
    pub fn with_sizes(builder: &mut Builder, value: Value, amount: Value) -> Self {
        let amount = trunc_or_zext(builder, amount, value.ty());
        ShlOp::new(builder, value, amount)
    }
}

pub(crate) fn clog2(value: usize) -> usize {
    usize::BITS as usize - value.next_power_of_two().leading_zeros() as usize - 1
}

pub(crate) fn type_width(ty: Type) -> usize {
    if is_array_type(ty) {
        array_type_size(ty)
    } else if is_integer_type(ty) {
        integer_type_width(ty)
    } else {
        panic!("unsupported indexing target type {}", ty)
    }
}

pub(crate) fn type_clog2(ty: Type) -> usize {
    clog2(type_width(ty))
}

pub(crate) fn trunc_or_zext_to_clog2(builder: &mut Builder, index: Value, into_ty: Type) -> Value {
    let target_width = std::cmp::max(type_clog2(into_ty), 1);
    trunc_or_zext(builder, index, get_integer_type(builder.cx, target_width))
}

pub(crate) fn trunc_or_zext(builder: &mut Builder, index: Value, into_ty: Type) -> Value {
    let target_width = type_width(into_ty);
    let actual_width = integer_type_width(index.ty());
    if target_width < actual_width {
        ExtractOp::with_sizes(builder, index, 0, target_width).into()
    } else if target_width > actual_width {
        let zero = ConstantOp::new(builder, target_width - actual_width, &BigInt::zero()).into();
        ConcatOp::new(builder, [zero, index].iter().copied()).into()
    } else {
        index
    }
}
