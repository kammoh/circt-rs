use crate::crate_prelude::*;
use hw::{OutputOp, ParamDeclAttr};
use itertools::{Either, Itertools};
use std::{borrow::Borrow, collections::HashMap};

def_operation!(HwModuleOp, "hw.module");

pub fn port_names<'a>(
    ctx: &'a Context,
    ports: impl Iterator<Item = impl Borrow<PortInfo>> + 'a,
) -> impl Iterator<Item = StringAttr> + 'a {
    ports.map(|pi| StringAttr::new(ctx, &pi.borrow().name))
}

pub fn port_types(
    ports: impl Iterator<Item = impl Borrow<PortInfo>>,
) -> impl Iterator<Item = Type> {
    ports.map(|pi| pi.borrow().ty)
}

impl HwModuleOp {
    /// Create a new module.
    pub fn build_with(
        builder: &mut OpBuilder,
        module: &Module,
        name: &str,
        ports: &ModulePortInfo,
        parameters: &[ParamDeclAttr],
        comment: &str,
        with_fn: impl FnOnce(
            &mut OpBuilder,
            &Block,
            &HashMap<String, Value>,
            &mut HashMap<String, Value>,
        ),
    ) -> Result<Self, Error> {
        let hw_module = Self::build_in_module(builder, module, name, ports, parameters, comment)?;
        let mut output_val_map = HashMap::default();
        let body = hw_module.first_block().ok_or(Error::IsNone)?;

        let inputs: HashMap<String, Value> =
            ports.inputs.iter().map(|pi| pi.name.clone()).zip(body.arguments()).collect();

        with_fn(builder, &body, &inputs, &mut output_val_map);

        let mut output_vals = Vec::default();
        for out_port in ports.outputs.iter() {
            output_vals.push(output_val_map.remove(&out_port.name).ok_or(Error::simple(
                format!("Value for output port: {} is missing!", &out_port.name),
            ))?);
        }
        match body.terminator() {
            Some(term) if hw::OutputOp::isa(&term) => {
                // already has OutputOp
            }
            _ => {
                builder.set_insertion_point(Some(InsertPoint::BlockEnd(body)));
                OutputOp::build::<Value>(builder, output_vals.iter()).ok_or(Error::IsNone)?;
            }
        };
        hw_module
            .verify()
            .then_some(hw_module)
            .ok_or(Error::simple(format!("hw::ModuleOp verification failed. {:?}", &hw_module)))
    }

    pub fn build(
        builder: &mut OpBuilder,
        name: &str,
        inputs: &[PortInfo],
        outputs: &[PortInfo],
        parameters: &[ParamDeclAttr],
        comment: &str,
    ) -> Result<Self, Error> {
        let region = Region::default();
        let block = Block::default();
        let op: Self = builder
            .build_with(|_, state| {
                let ctx = builder.context();
                region.append_block(&block);
                state.add_region(&region);

                for i in inputs {
                    block.add_argument(&i.borrow().ty, builder.loc());
                }
                state.add_attribute(SymbolTable::symbol_attr_name(), &StringAttr::new(ctx, name));
                state.add_attribute(
                    "argNames",
                    &ArrayAttr::new(ctx, port_names(ctx, inputs.iter())),
                );
                state.add_attribute(
                    "resultNames",
                    &ArrayAttr::new(ctx, port_names(ctx, outputs.iter())),
                );
                state.add_attribute(
                    "parameters",
                    &ArrayAttr::new::<ParamDeclAttr>(ctx, parameters.iter()),
                );
                state.add_attribute(
                    "function_type",
                    &TypeAttr::new(&FunctionType::new(
                        ctx,
                        port_types(inputs.iter()),
                        port_types(outputs.iter()),
                    )),
                );
                state.add_attribute("comment", &StringAttr::new(ctx, comment));
            })
            .expect("OpBuilder failed");
        let body = op.first_block().ok_or(Error::IsNone)?;
        builder.set_insertion_point(Some(InsertPoint::BlockEnd(body)));
        Ok(op)
    }

    pub fn build_in_module(
        builder: &mut OpBuilder,
        module: &Module,
        name: &str,
        ports: &ModulePortInfo,
        parameters: &[ParamDeclAttr],
        comment: &str,
    ) -> Result<Self, Error> {
        builder.set_insertion_point(Some(InsertPoint::BlockEnd(module.body())));
        Self::build(builder, name, &ports.inputs, &ports.outputs, parameters, comment)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PortDirection {
    Input = 1,
    Output = 2,
    InOut = 3,
}

impl PortDirection {
    pub fn flip(self) -> Self {
        match self {
            PortDirection::Input => PortDirection::Output,
            PortDirection::Output => PortDirection::Input,
            PortDirection::InOut => PortDirection::InOut,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PortInfo {
    pub name: String,
    pub direction: PortDirection,
    pub ty: Type,
    // pub arg_num: usize, // order
}

impl PortInfo {
    pub fn new(direction: PortDirection, name: &str, ty: &impl Ty) -> Self {
        Self {
            name: name.to_string(),
            direction,
            ty: ty.as_type(),
        }
    }
    pub fn input(name: &str, ty: &impl Ty) -> Self {
        Self::new(PortDirection::Input, name, ty)
    }
    pub fn output(name: &str, ty: &impl Ty) -> Self {
        Self::new(PortDirection::Output, name, ty)
    }

    // pub fn id(&self) -> isize {
    //     let arg_num = self.arg_num as isize;
    //     match self.direction {
    //         PortDirection::Output => arg_num,
    //         _ => -1 - arg_num,
    //     }
    // }
    pub fn is_output(&self) -> bool {
        matches!(self.direction, PortDirection::Output)
    }
}

#[derive(Clone, Default)]
pub struct ModulePortInfo {
    pub inputs: Vec<PortInfo>,
    pub outputs: Vec<PortInfo>,
}

impl ModulePortInfo {
    pub fn from_merged(merged_ports: impl IntoIterator<Item = PortInfo>) -> Self {
        let (outputs, inputs): (Vec<_>, Vec<_>) =
            merged_ports.into_iter().partition_map(|pi| match pi.direction {
                PortDirection::Output => Either::Left(pi),
                _ => Either::Right(pi),
            });
        Self { inputs, outputs }
    }

    pub fn add_input(&mut self, name: &str, ty: &impl Ty) {
        self.inputs.push(PortInfo::input(name, ty))
    }

    pub fn add_output(&mut self, name: &str, ty: &impl Ty) {
        self.outputs.push(PortInfo::output(name, ty))
    }
}

#[cfg(test)]
mod tests {}
