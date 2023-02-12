// Copyright (c) 2016-2021 Fabian Schuiki
// Copyright (c) 2022-2023 Kamyar Mohajerani

use crate::crate_prelude::*;

define_dialect!(func);

def_operation!(FuncOp, "func.func");
def_operation!(CallOp, "func.call");
def_operation!(ReturnOp, "func.return");

pub struct FunctionBuilder<'a> {
    name: &'a str,
    args: Vec<(String, Type)>,
    results: Vec<(String, Type)>,
}

impl SingleRegionOp for FuncOp {}

impl FuncOp {
    /// Get the number of arguments.
    pub fn num_arguments(&self) -> usize {
        self.first_block().unwrap().num_arguments()
    }

    /// Get an argument by index.
    pub fn argument(&self, index: usize) -> Option<Value> {
        self.first_block().unwrap().argument(index)
    }

    /// Get an iterator over all arguments.
    pub fn arguments(&self) -> impl Iterator<Item = Value> + '_ {
        (0..self.num_arguments()).map(|i| self.argument(i).unwrap())
    }
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            args: vec![],
            results: vec![],
        }
    }

    /// Add an argument.
    pub fn add_arg(&mut self, name: Option<String>, ty: Type) -> &mut Self {
        self.args.push((name.unwrap_or("".to_string()), ty));
        self
    }

    /// Add a result.
    pub fn add_result(&mut self, name: Option<String>, ty: Type) -> &mut Self {
        self.results.push((name.unwrap_or("".to_string()), ty));
        self
    }

    /// Build a function.
    pub fn build(&mut self, builder: &mut OpBuilder) -> FuncOp {
        builder
            .build_with(|builder, state| {
                let ctx = builder.context();
                let arg_types = self.args.iter().map(|(_, ty)| ty.clone());
                let result_types = self.results.iter().map(|(_, ty)| ty.clone());
                // let mlir_arg_types: Vec<MlirType> = arg_types.clone().map(|x| x.raw()).collect();
                // let mlir_result_types: Vec<MlirType> =
                //     result_types.clone().map(|x| x.raw()).collect();
                let arg_names: Vec<_> = self
                    .args
                    .iter()
                    .map(|(name, _)| StringAttr::new(ctx, name))
                    .collect();

                state.add_attribute("sym_name", &StringAttr::new(ctx, self.name));
                state.add_attribute(
                    "function_type",
                    &TypeAttr::new(&ctx.get_function_type(arg_types, result_types).unwrap()),
                );
                state.add_attribute("arg_names", &ArrayAttr::new(ctx, arg_names));
                let region = Region::default();
                // let locations = vec![Location::new_unknown(ctx); mlir_arg_types.len()];
                let block = Block::default();
                region.append_block(&block);

                state.add_region(&region);
            })
            .unwrap()
    }
}

// impl CallOp {
//     /// Create a new call.
//     pub fn new(
//         builder: &mut OpBuilder,
//         callee: &str,
//         args: impl IntoIterator<Item = Value>,
//         results: impl IntoIterator<Item = Type>,
//     ) -> Self {
//         builder.build_with(|builder, state| {
//             let _num_args = args.into_iter().map(|v| state.add_operand(v)).count();
//             let _num_results = results.into_iter().map(|v| state.add_result(v)).count();
//             state.add_attribute("callee", &get_flat_symbol_ref_attr(builder.ctx, callee));
//         })
//     }
// }

// impl ReturnOp {
//     /// Create a new return.
//     pub fn new(builder: &mut OpBuilder, values: impl IntoIterator<Item = Value>) -> Self {
//         builder.build_with(|_, state| {
//             for v in values {
//                 state.add_operand(v);
//             }
//         })
//     }
// }
