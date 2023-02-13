// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;
use circt_sys::cxx_bindings;
use simple_error::SimpleError;
use std::borrow::Borrow;

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

def_operation_many_to_one!(AndOp, "comb.and");
def_operation_many_to_one!(OrOp, "comb.or");
def_operation_many_to_one!(XorOp, "comb.xor");

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
    pub fn new(builder: &mut OpBuilder, pred: CmpPred, lhs: &Value, rhs: &Value) -> Option<Self> {
        builder.build_with(|builder, state| {
            let ctx = builder.context();
            state.add_operand(lhs);
            state.add_operand(rhs);
            let attr = IntegerAttr::new(&IntegerType::new(ctx, 64), pred as u32);
            state.add_attribute("predicate", &attr);
            state.add_result(&IntegerType::new(ctx, 1));
        })
    }
}

impl MuxOp {
    /// Create a new mux operation.
    pub fn build(
        builder: &mut OpBuilder,
        cond: &Value,
        true_value: &Value,
        false_value: &Value,
    ) -> Option<Self> {
        builder.build_with(|_, state| {
            state.add_operand(cond);
            state.add_operand(true_value);
            state.add_operand(false_value);
            state.add_result(&true_value.ty());
        })
    }
}

impl ExtractOp {
    /// Extract a bit range from an integer.
    pub fn build(
        builder: &mut OpBuilder,
        ty: &impl Ty,
        value: &impl Val,
        offset: usize,
    ) -> Option<Self> {
        builder.build_with(|builder, state| {
            state.add_operand(value);
            let i32 = IntegerType::new(builder.context(), 32);
            state.add_attribute("lowBit", &IntegerAttr::new(&i32, offset as i64));
            state.add_result(ty);
        })
    }

    pub fn with_sizes(
        builder: &mut OpBuilder,
        value: &Value,
        offset: usize,
        length: usize,
    ) -> Option<Self> {
        Self::build(
            builder,
            &IntegerType::new(builder.context(), length.try_into().ok()?),
            value,
            offset,
        )
    }
}

impl ConcatOp {
    pub fn build(
        builder: &mut OpBuilder,
        values: impl IntoIterator<Item = impl Borrow<Value>>,
    ) -> Option<Self> {
        builder.build_with(|builder, state| {
            let mut width = 0;
            for value in values {
                let value = value.borrow();
                state.add_operand(value);
                let ty = IntegerType::try_from(value.ty()).unwrap();
                width += ty.width();
            }
            state.add_result(&IntegerType::new(builder.context(), width));
        })
    }
}

impl ShrUOp {
    pub fn with_sizes(builder: &mut OpBuilder, value: &Value, amount: &Value) -> Option<Self> {
        let amount = trunc_or_zext(builder, amount, &value.ty())?;
        ShrUOp::build(builder, value, &amount)
    }
}

impl ShrSOp {
    pub fn with_sizes(builder: &mut OpBuilder, value: &Value, amount: &Value) -> Option<Self> {
        let amount = trunc_or_zext(builder, amount, &value.ty())?;
        ShrSOp::build(builder, value, &amount)
    }
}

impl ShlOp {
    pub fn with_sizes(builder: &mut OpBuilder, value: &Value, amount: &Value) -> Option<Self> {
        let amount = trunc_or_zext(builder, amount, &value.ty())?;
        ShlOp::build(builder, value, &amount)
    }
}

pub(crate) fn clog2(value: usize) -> usize {
    usize::BITS as usize - value.next_power_of_two().leading_zeros() as usize - 1
}

pub(crate) fn type_width(ty: &Type) -> Result<usize, SimpleError> {
    if let Ok(ty) = hw::ArrayType::try_from(ty) {
        Ok(ty.size())
    } else if let Ok(ty) = IntegerType::try_from(ty) {
        Ok(ty.width() as _)
    } else {
        Err(SimpleError::new(format!(
            "unsupported indexing target type {}",
            ty
        )))
    }
}

pub(crate) fn type_clog2(ty: &impl Ty) -> usize {
    clog2(type_width(&ty.into()).unwrap())
}

pub(crate) fn trunc_or_zext_to_clog2(
    builder: &mut OpBuilder,
    index: &Value,
    into_ty: &impl Ty,
) -> Option<Value> {
    trunc_or_zext(
        builder,
        index,
        &IntegerType::new(builder.context(), type_clog2(into_ty).max(1) as _).into(),
    )
}

pub(crate) fn trunc_or_zext(
    builder: &mut OpBuilder,
    index: &Value,
    into_ty: &Type,
) -> Option<Value> {
    let target_width: u32 = type_width(into_ty).ok()?.try_into().ok()?;
    let ty = IntegerType::try_from(index.ty()).ok()?;
    let actual_width = ty.width();
    match target_width.cmp(&actual_width) {
        std::cmp::Ordering::Less => {
            ExtractOp::with_sizes(builder, index, 0, target_width as _)?.result_at(0)
        }
        std::cmp::Ordering::Greater => {
            let zero = hw::ConstantOp::build(builder, target_width - actual_width, 0)?.result();
            ConcatOp::build(builder, [zero, index.clone()].iter().cloned())?.result_at(0)
        }
        std::cmp::Ordering::Equal => Some(index.clone()),
    }
}
