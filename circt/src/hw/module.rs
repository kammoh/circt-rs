use std::{borrow::Borrow, collections::HashMap};

use crate::crate_prelude::*;
use itertools::{Either, Itertools};

use hw::{OutputOp, ParamDeclAttr};

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
    ) -> Option<Self> {
        let hw_module = Self::build_in_module(builder, module, name, ports, parameters, comment)?;
        let mut output_val_map = HashMap::new();
        let mut output_vals = Vec::new();

        let body = hw_module.first_block()?;

        let inputs: HashMap<String, Value> = ports
            .inputs
            .iter()
            .map(|pi| pi.name.clone())
            .zip(body.arguments())
            .collect();

        with_fn(builder, &body, &inputs, &mut output_val_map);

        for out_port in ports.outputs.iter() {
            if let Some(val) = output_val_map.remove(&out_port.name) {
                output_vals.push(val);
            } else {
                eprintln!("Value for output port: {} is missing!", &out_port.name);
                return None;
            }
        }
        match body.terminator() {
            Some(term) if hw::OutputOp::isa(&term) => {
                // already has OutputOp
            }
            _ => {
                builder.set_insertion_point(Some(InsertPoint::BlockEnd(body)));
                OutputOp::build::<Value>(builder, output_vals.iter())?;
            }
        };
        hw_module.verify().then_some(hw_module)
    }

    pub fn build(
        builder: &mut OpBuilder,
        name: &str,
        inputs: &[PortInfo],
        outputs: &[PortInfo],
        parameters: &[ParamDeclAttr],
        comment: &str,
    ) -> Option<Self> {
        let region = Region::new();
        let block = Block::new();
        let op: Self = builder
            .build_with(|builder, state| {
                let ctx = builder.context();
                region.append_block(&block);
                state.add_region(&region);

                for i in inputs {
                    block.add_argument(&i.borrow().ty, builder.loc());
                }
                state.add_attribute(SymbolTable::symbol_attr_name(), &StringAttr::new(ctx, name));
                state.add_attribute(
                    "argNames",
                    &ArrayAttr::new(ctx, port_names(&ctx, inputs.iter())),
                );
                state.add_attribute(
                    "resultNames",
                    &ArrayAttr::new(ctx, port_names(&ctx, outputs.iter())),
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
        dbg!(op);
        let body = op.first_block()?;
        builder.set_insertion_point(Some(InsertPoint::BlockEnd(body)));
        Some(op)
    }

    pub fn build_in_module(
        builder: &mut OpBuilder,
        module: &Module,
        name: &str,
        ports: &ModulePortInfo,
        parameters: &[ParamDeclAttr],
        comment: &str,
    ) -> Option<Self> {
        builder.set_insertion_point(Some(InsertPoint::BlockEnd(module.body())));
        Self::build(
            builder,
            name,
            &ports.inputs,
            &ports.outputs,
            parameters,
            comment,
        )
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
        match self.direction {
            PortDirection::Output => true,
            _ => false,
        }
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
            merged_ports
                .into_iter()
                .partition_map(|pi| match pi.direction {
                    PortDirection::Output => Either::Left(pi.clone()),
                    _ => Either::Right(pi.clone()),
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
mod tests {
    use crate::{hw::ModulePortInfo, *};
    #[test]
    fn test_module_build() {
        let ctx = OwnedContext::new();
        comb::dialect().unwrap().load_dialect(&ctx).unwrap();
        seq::dialect().unwrap().load_dialect(&ctx).unwrap();
        hw::dialect().unwrap().load_dialect(&ctx).unwrap();
        fsm::dialect().unwrap().load_dialect(&ctx).unwrap();
        sv::dialect().unwrap().load_dialect(&ctx).unwrap();
        seq::register_seq_passes();
        hw::register_hw_arith_passes();
        fsm::register_fsm_passes();
        sv::register_sv_passes();

        let mut builder = OpBuilder::new(&ctx);
        let module = Module::create(builder.loc());

        let hw_module_name = "test_hw_module";
        let mut ports = ModulePortInfo::default();

        let i1 = IntegerType::new(&ctx, 1);

        ports.add_input("a", &i1);
        ports.add_input("b", &i1);
        ports.add_output("c", &i1);
        ports.add_output("c1", &i1);

        let hw_module = hw::HwModuleOp::build_with(
            &mut builder,
            &module,
            hw_module_name,
            &ports,
            &[],
            "no comments!",
            |builder, _, inputs, outputs| {
                let c1 = hw::ConstantOp::build(builder, 1, 1)
                    .unwrap()
                    .result(0)
                    .unwrap();
                outputs.insert("c1".to_string(), c1);

                let c = comb::AndOp::build(builder, &[inputs["a"], inputs["b"]])
                    .unwrap()
                    .result(0)
                    .unwrap();

                outputs.insert("c".to_string(), c);
            },
        )
        .unwrap();

        println!("got hw_module: {:?}", hw_module);

        assert!(hw_module.verify(), "hw_module verify failed");

        let pm = PassManager::new(&ctx);

        let passes = vec![
            "builtin.module(lower-hwarith-to-hw)",
            "builtin.module(hw.module(lower-seq-hlmem))",
            "builtin.module(convert-fsm-to-sv)",
            "builtin.module(lower-seq-to-sv)",
            // "builtin.module(cse, canonicalize, cse)",
            "builtin.module(hw.module(prettify-verilog), hw.module(hw-cleanup))",
        ];

        for pipeline in passes.iter() {
            println!("Running pass: {}", pipeline);
            pm.parse(pipeline).expect("parse failed");
            let r = pm.run(&module);
            assert!(r.is_success());
        }

        println!("hw_module: {:?}", module.operation());

        let out_dir = Path::new(hw_module_name);
        std::fs::create_dir_all(out_dir).unwrap();
        export_split_verilog(&module, &out_dir);

    }
}
