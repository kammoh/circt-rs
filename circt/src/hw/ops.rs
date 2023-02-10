use std::borrow::Borrow;

use super::*;
use crate::crate_prelude::*;

use num::{BigInt, Num};

def_operation!(ConstantOp, "hw.constant");

impl ConstantOp {
    /// Create a new constant value.
    pub fn build(
        builder: &mut OpBuilder,
        width: u32,
        value: impl Num + std::fmt::Display,
    ) -> Option<Self> {
        builder.build_with(|builder, state| {
            let ty = IntegerType::new(builder.context(), width);
            state.add_attribute(
                "value",
                &IntegerAttr::from_str(&ty, value.to_string().as_str()),
            );
            state.add_result(&ty);
        })
    }
}

def_operation_single_result!(ArrayCreateOp, "hw.array_create");
def_operation_single_result!(StructCreateOp, "hw.struct_create");
def_binary_operation_explicit_result!(ArraySliceOp, "hw.array_slice");
def_operation_single_result!(ArrayConcatOp, "hw.array_concat");
def_operation_single_result!(ArrayGetOp, "hw.array_get");
def_operation_single_result!(StructExtractOp, "hw.struct_extract");
def_operation_single_result!(StructInjectOp, "hw.struct_inject");
def_operation_single_result!(BitcastOp, "hw.bitcast");

def_operation!(OutputOp, "hw.output"; doc = "HW termination operation. It marks the end of a region in the HW dialect and the values to put on the output ports."); // n-args, zero result

impl OutputOp {
    /// Create a new output.
    pub fn build<V: Val>(
        builder: &mut OpBuilder,
        outputs: impl IntoIterator<Item = impl Borrow<V>>,
    ) -> Option<Self> {
        builder.build_with(|_, state| {
            state.add_operands(outputs);
        })
    }
}

def_operation!(InstanceOp, "hw.instance"); // n-args, m-results

// impl ArrayCreateOp {
//     /// Create a new array value.
//     pub fn new(builder: &mut OpBuilder, ty: Type, values: impl IntoIterator<Item = Value>) -> Self {
//         builder.build_with(|_, state| {
//             for value in values {
//                 state.add_operand(value);
//             }
//             state.add_result(ty);
//         })
//     }
// }

// impl StructCreateOp {
//     /// Create a new struct value.
//     pub fn new(builder: &mut OpBuilder, ty: Type, values: impl IntoIterator<Item = Value>) -> Self {
//         builder.build_with(|_, state| {
//             for value in values {
//                 state.add_operand(value);
//             }
//             state.add_result(ty);
//         })
//     }
// }

// impl ArraySliceOp {
//     pub fn with_sizes(builder: &mut OpBuilder, value: Value, offset: Value, length: usize) -> Self {
//         let ty = value.ty().unwrap();
//         let offset = trunc_or_zext_to_clog2(builder, offset, ty);
//         Self::new(
//             builder,
//             get_array_type(&array_type_element(ty).unwrap(), length).unwrap(),
//             value,
//             offset,
//         )
//     }

//     pub fn with_const_offset(
//         builder: &mut OpBuilder,
//         value: Value,
//         offset: usize,
//         length: usize,
//     ) -> Self {
//         let offset = crate::hw::ConstantOp::new(builder, 64, &offset.into())
//             .result(0)
//             .unwrap();
//         Self::with_sizes(builder, value, offset, length)
//     }
// }

// impl ArrayConcatOp {
//     pub fn new(builder: &mut OpBuilder, values: impl IntoIterator<Item = Value>) -> Self {
//         builder.build_with(|_, state| {
//             let mut width = 0;
//             let mut element_ty = None;
//             for value in values {
//                 state.add_operand(value);
//                 let ty = value.ty().unwrap();
//                 width += array_type_size(ty);
//                 element_ty = array_type_element(ty);
//             }
//             state.add_result(get_array_type(&element_ty.unwrap(), width).unwrap());
//         })
//     }
// }

// impl ArrayGetOp {
//     pub fn new(builder: &mut OpBuilder, value: Value, offset: Value) -> Self {
//         let ty = value.ty().unwrap();
//         let offset = trunc_or_zext_to_clog2(builder, offset, ty);
//         builder.build_with(|_, state| {
//             state.add_operand(value);
//             state.add_operand(offset);
//             state.add_result(array_type_element(ty).unwrap());
//         })
//     }

//     pub fn with_const_offset(builder: &mut OpBuilder, value: Value, offset: usize) -> Self {
//         let offset = crate::hw::ConstantOp::new(builder, 64, &offset.into())
//             .result(0)
//             .unwrap();
//         Self::new(builder, value, offset)
//     }
// }

// impl StructExtractOp {
//     pub fn new(builder: &mut OpBuilder, value: Value, offset: usize) -> Self {
//         builder.build_with(|_, state| {
//             state.add_operand(value);
//             let (field_name, field_ty) = struct_type_field(value.ty().unwrap(), offset);
//             state.add_attribute("field", &get_string_attr(builder.ctx, &field_name));
//             state.add_result(field_ty);
//         })
//     }
// }

// impl StructInjectOp {
//     pub fn new(builder: &mut OpBuilder, value: Value, field_value: Value, offset: usize) -> Self {
//         let ty = value.ty().unwrap();
//         builder.build_with(|_, state| {
//             state.add_operand(value);
//             state.add_operand(field_value);
//             let (field_name, _) = struct_type_field(ty, offset);
//             state.add_attribute("field", &get_string_attr(builder.ctx, &field_name));
//             state.add_result(ty);
//         })
//     }
// }

// impl BitcastOp {
//     /// Create a new bitcast.
//     pub fn new(builder: &mut OpBuilder, ty: Type, value: Value) -> Self {
//         builder.build_with(move |_, state| {
//             state.add_operand(value);
//             state.add_result(ty);
//         })
//     }
// }
